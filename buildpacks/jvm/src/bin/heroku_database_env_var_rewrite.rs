// Required due to: https://github.com/rust-lang/rust/issues/95513
#![allow(unused_crate_dependencies)]

use libcnb::data::exec_d::ExecDProgramOutputKey;
use libcnb::exec_d::write_exec_d_program_output;
use std::collections::HashMap;
use url::Url;

fn main() {
    write_exec_d_program_output(
        jvm_env_vars_for_env(&std::env::vars().collect())
            .expect("Heroku database environment variables should be rewritable")
            .into_iter()
            .filter_map(|(key, value)| key.parse().ok().map(|key| (key, value)))
            .collect::<HashMap<ExecDProgramOutputKey, String>>(),
    );
}

fn jvm_env_vars_for_env(
    input: &HashMap<String, String>,
) -> Result<HashMap<String, String>, DatabaseEnvVarError> {
    let mut result = HashMap::new();

    if let Some(database_url) = input.get("DATABASE_URL") {
        result.extend(env_vars_for_database_url(database_url, DEFAULT_ENV_PREFIX)?);

        // This handling might look wrong at first, but this is how it was historically implemented.
        // The DATABASE_CONNECTION_POOL_URL will only be considered with DATABASE_URL is set as
        // well. To not break existing customers, this handling is re-implemented in the same way.
        if let Some(database_connection_pool_url) = input.get("DATABASE_CONNECTION_POOL_URL") {
            result.extend(env_vars_for_database_url(
                database_connection_pool_url,
                DEFAULT_ENV_PREFIX,
            )?);

            result.extend(env_vars_for_database_url(
                database_connection_pool_url,
                "DATABASE_CONNECTION_POOL_JDBC",
            )?);
        }

        // Handling for Spring specific JDBC environment variables
        let disable_spring_datasource_url = input
            .get("DISABLE_SPRING_DATASOURCE_URL")
            .map_or(false, |value| value == "true");

        if !disable_spring_datasource_url
            && !input.contains_key("SPRING_DATASOURCE_URL")
            && !input.contains_key("SPRING_DATASOURCE_USERNAME")
            && !input.contains_key("SPRING_DATASOURCE_PASSWORD")
        {
            result.extend(env_vars_for_database_url(
                database_url,
                "SPRING_DATASOURCE",
            )?);
        }
    } else {
        // If there is no DATABASE_URL, we try some known environment variables from third-party
        // Heroku database addons and treat them like DATABASE_URL.
        for third_party_database_url_env_var in THIRD_PARTY_DATABASE_URL_ENV_VARS {
            if let Some(third_party_database_url) = input.get(*third_party_database_url_env_var) {
                result.extend(env_vars_for_database_url(
                    third_party_database_url,
                    DEFAULT_ENV_PREFIX,
                )?);
            }
        }
    }

    // When multiple databases are attached to an app, Heroku addons will set additional environment
    // variables for each database, prefixed with a known string. To make using them easy with JDBC,
    // we emit JDBC compatible environment variables for them as well.
    for (name, value) in input
        .iter()
        .filter(|(name, _)| name.starts_with("HEROKU_POSTGRESQL_") && name.ends_with("_URL"))
    {
        result.extend(env_vars_for_database_url(
            value,
            format!("{}_JDBC", name.strip_suffix("_URL").unwrap_or(name)),
        )?);
    }

    // Spring uses a dedicated environment variable when connecting to Redis. If that environment
    // variable is not already set, we copy the value from the Heroku REDIS_URL into
    // SPRING_REDIS_URL for convenience.
    if !input.contains_key("DISABLE_SPRING_REDIS_URL") && !input.contains_key("SPRING_REDIS_URL") {
        if let Some(redis_url) = input.get("REDIS_URL") {
            result.insert(String::from("SPRING_REDIS_URL"), redis_url.clone());
        }
    }

    Ok(result)
}

fn env_vars_for_database_url(
    url_string: impl AsRef<str>,
    env_var_prefix: impl AsRef<str>,
) -> Result<HashMap<String, String>, DatabaseEnvVarError> {
    let mut url = Url::parse(url_string.as_ref())?;

    // Previous versions of this script only set the environment variables when a username and
    // password was present. We keep this logic to ensure backwards compatability.
    let original_username = match url.username() {
        "" => return Ok(HashMap::new()),
        username => String::from(username),
    };

    let original_password = match url.password() {
        None => return Ok(HashMap::new()),
        Some(password) => String::from(password),
    };

    url.set_username("")
        .map_err(|()| DatabaseEnvVarError::CannotSetUsername)?;

    url.set_password(None)
        .map_err(|()| DatabaseEnvVarError::CannotSetPassword)?;

    url.query_pairs_mut()
        .append_pair("user", &original_username)
        .append_pair("password", &original_password);

    if url.scheme() == "postgres" {
        url.set_scheme("postgresql")
            .map_err(|()| DatabaseEnvVarError::CannotSetScheme)?;
        url.query_pairs_mut().append_pair("sslmode", "require");
    };

    Ok(HashMap::from([
        (
            format!("{}_URL", env_var_prefix.as_ref()),
            format!("jdbc:{url}"),
        ),
        (
            format!("{}_USERNAME", env_var_prefix.as_ref()),
            original_username,
        ),
        (
            format!("{}_PASSWORD", env_var_prefix.as_ref()),
            original_password,
        ),
    ]))
}

#[derive(thiserror::Error, Debug)]
enum DatabaseEnvVarError {
    #[error(transparent)]
    CannotParseUrl(#[from] url::ParseError),
    #[error("Cannot set username in database URL")]
    CannotSetUsername,
    #[error("Cannot set password in database URL")]
    CannotSetPassword,
    #[error("Cannot set scheme in database URL")]
    CannotSetScheme,
}

const DEFAULT_ENV_PREFIX: &str = "JDBC_DATABASE";
const THIRD_PARTY_DATABASE_URL_ENV_VARS: &[&str] =
    &["JAWSDB_URL", "JAWSDB_MARIA_URL", "CLEARDB_DATABASE_URL"];

#[cfg(test)]
mod tests {
    use crate::jvm_env_vars_for_env;
    use std::collections::HashMap;

    #[test]
    fn default_database_env_var() {
        let result = jvm_env_vars_for_env(&HashMap::from([(
            String::from("DATABASE_URL"),
            String::from("postgres://AzureDiamond:hunter2@db.example.com:5432/testdb"),
        )]))
        .unwrap();

        assert_eq!(
            result.get("JDBC_DATABASE_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432/testdb?user=AzureDiamond&password=hunter2&sslmode=require"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_USERNAME"),
            Some(&String::from("AzureDiamond"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_PASSWORD"),
            Some(&String::from("hunter2"))
        );
    }

    #[test]
    fn color_database_env_var() {
        let result = jvm_env_vars_for_env(&HashMap::from([
            (
                String::from("HEROKU_POSTGRESQL_RED_URL"),
                String::from("postgres://red:charmander@db.example.com:5432/fire-pokemon"),
            ),
            (
                String::from("HEROKU_POSTGRESQL_BLUE_URL"),
                String::from("postgres://blue:squirtle@db.example.com:5432/water-pokemon"),
            ),
        ]))
        .unwrap();

        assert_eq!(
            result.get("HEROKU_POSTGRESQL_RED_JDBC_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432/fire-pokemon?user=red&password=charmander&sslmode=require"))
        );

        assert_eq!(
            result.get("HEROKU_POSTGRESQL_RED_JDBC_USERNAME"),
            Some(&String::from("red"))
        );
        assert_eq!(
            result.get("HEROKU_POSTGRESQL_RED_JDBC_PASSWORD"),
            Some(&String::from("charmander"))
        );

        assert_eq!(
            result.get("HEROKU_POSTGRESQL_BLUE_JDBC_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432/water-pokemon?user=blue&password=squirtle&sslmode=require"))
        );

        assert_eq!(
            result.get("HEROKU_POSTGRESQL_BLUE_JDBC_USERNAME"),
            Some(&String::from("blue"))
        );
        assert_eq!(
            result.get("HEROKU_POSTGRESQL_BLUE_JDBC_PASSWORD"),
            Some(&String::from("squirtle"))
        );
    }

    #[test]
    fn mysql_env_var() {
        let result = jvm_env_vars_for_env(&HashMap::from([(
            String::from("DATABASE_URL"),
            String::from("mysql://foo:bar@ec2-0-0-0-0:5432/abc123?reconnect=true"),
        )]))
        .unwrap();

        assert_eq!(
            result.get("JDBC_DATABASE_URL"),
            Some(&String::from(
                "jdbc:mysql://ec2-0-0-0-0:5432/abc123?reconnect=true&user=foo&password=bar"
            ))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_USERNAME"),
            Some(&String::from("foo"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_PASSWORD"),
            Some(&String::from("bar"))
        );
    }

    #[test]
    fn third_party_database_urls() {
        for database_url_var_name in ["JAWSDB_URL", "JAWSDB_MARIA_URL", "CLEARDB_DATABASE_URL"] {
            let result = jvm_env_vars_for_env(&HashMap::from([(
                String::from(database_url_var_name),
                format!(
                    "mysql://foo:bar@ec2-0-0-0-0:5432/{}?reconnect=true",
                    &database_url_var_name
                ),
            )]))
            .unwrap();

            assert_eq!(
                result.get("JDBC_DATABASE_URL"),
                Some(&format!(
                    "jdbc:mysql://ec2-0-0-0-0:5432/{}?reconnect=true&user=foo&password=bar",
                    &database_url_var_name
                ))
            );

            assert_eq!(
                result.get("JDBC_DATABASE_USERNAME"),
                Some(&String::from("foo"))
            );

            assert_eq!(
                result.get("JDBC_DATABASE_PASSWORD"),
                Some(&String::from("bar"))
            );
        }
    }

    #[test]
    fn third_party_database_urls_priority() {
        for database_url_var_name in ["JAWSDB_URL", "JAWSDB_MARIA_URL", "CLEARDB_DATABASE_URL"] {
            let result = jvm_env_vars_for_env(&HashMap::from([
                (
                    String::from(database_url_var_name),
                    format!(
                        "mysql://foo:bar@ec2-0-0-0-0:5432/{}?reconnect=true",
                        &database_url_var_name
                    ),
                ),
                (
                    String::from("DATABASE_URL"),
                    String::from(
                        "postgres://AzureDiamond:hunter2@db.example.com:5432/regular-database",
                    ),
                ),
            ]))
            .unwrap();

            assert_eq!(
                result.get("JDBC_DATABASE_URL"),
                Some(&String::from("jdbc:postgresql://db.example.com:5432/regular-database?user=AzureDiamond&password=hunter2&sslmode=require"))
            );

            assert_eq!(
                result.get("JDBC_DATABASE_USERNAME"),
                Some(&String::from("AzureDiamond"))
            );

            assert_eq!(
                result.get("JDBC_DATABASE_PASSWORD"),
                Some(&String::from("hunter2"))
            );
        }
    }

    #[test]
    fn database_connection_pool() {
        let result = jvm_env_vars_for_env(&HashMap::from([
            (
                String::from("DATABASE_URL"),
                String::from("postgres://AzureDiamond:hunter2@db.example.com:5432/testdb"),
            ),
            (
                String::from("DATABASE_CONNECTION_POOL_URL"),
                String::from("postgres://pooluser:poolpass@pooled.example.com:5432/testdb"),
            ),
        ]))
        .unwrap();

        assert_eq!(
            result.get("JDBC_DATABASE_URL"),
            Some(&String::from("jdbc:postgresql://pooled.example.com:5432/testdb?user=pooluser&password=poolpass&sslmode=require"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_USERNAME"),
            Some(&String::from("pooluser"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_PASSWORD"),
            Some(&String::from("poolpass"))
        );

        assert_eq!(
            result.get("DATABASE_CONNECTION_POOL_JDBC_URL"),
            Some(&String::from("jdbc:postgresql://pooled.example.com:5432/testdb?user=pooluser&password=poolpass&sslmode=require"))
        );

        assert_eq!(
            result.get("DATABASE_CONNECTION_POOL_JDBC_USERNAME"),
            Some(&String::from("pooluser"))
        );

        assert_eq!(
            result.get("DATABASE_CONNECTION_POOL_JDBC_PASSWORD"),
            Some(&String::from("poolpass"))
        );
    }

    #[test]
    fn database_connection_pool_without_database_url() {
        let result = jvm_env_vars_for_env(&HashMap::from([(
            String::from("DATABASE_CONNECTION_POOL_URL"),
            String::from("postgres://pooluser:poolpass@pooled.example.com:5432/testdb"),
        )]))
        .unwrap();

        assert_eq!(result, HashMap::new());
    }

    #[test]
    fn spring_datasource_support() {
        let result = jvm_env_vars_for_env(&HashMap::from([(
            String::from("DATABASE_URL"),
            String::from("postgres://AzureDiamond:hunter2@db.example.com:5432/testdb"),
        )]))
        .unwrap();

        assert_eq!(
            result.get("JDBC_DATABASE_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432/testdb?user=AzureDiamond&password=hunter2&sslmode=require"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_USERNAME"),
            Some(&String::from("AzureDiamond"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_PASSWORD"),
            Some(&String::from("hunter2"))
        );

        assert_eq!(
            result.get("SPRING_DATASOURCE_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432/testdb?user=AzureDiamond&password=hunter2&sslmode=require"))
        );

        assert_eq!(
            result.get("SPRING_DATASOURCE_USERNAME"),
            Some(&String::from("AzureDiamond"))
        );

        assert_eq!(
            result.get("SPRING_DATASOURCE_PASSWORD"),
            Some(&String::from("hunter2"))
        );
    }

    #[test]
    fn spring_datasource_support_explicitly_disabled() {
        let result = jvm_env_vars_for_env(&HashMap::from([
            (
                String::from("DATABASE_URL"),
                String::from("postgres://AzureDiamond:hunter2@db.example.com:5432/testdb"),
            ),
            (
                String::from("DISABLE_SPRING_DATASOURCE_URL"),
                String::from("true"),
            ),
        ]))
        .unwrap();

        assert_eq!(
            result.get("JDBC_DATABASE_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432/testdb?user=AzureDiamond&password=hunter2&sslmode=require"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_USERNAME"),
            Some(&String::from("AzureDiamond"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_PASSWORD"),
            Some(&String::from("hunter2"))
        );

        assert_eq!(result.get("SPRING_DATASOURCE_URL"), None);
        assert_eq!(result.get("SPRING_DATASOURCE_USERNAME"), None);
        assert_eq!(result.get("SPRING_DATASOURCE_PASSWORD"), None);
    }

    #[test]
    fn spring_datasource_support_implicitly_disabled() {
        let result = jvm_env_vars_for_env(&HashMap::from([
            (
                String::from("DATABASE_URL"),
                String::from("postgres://AzureDiamond:hunter2@db.example.com:5432/testdb"),
            ),
            (
                String::from("SPRING_DATASOURCE_URL"),
                String::from("jdbc:postgresql://db.example.com:5432/testdb?user=AzureDiamondSpring&password=hunter2&sslmode=require"),
            ),
        ])).unwrap();

        assert_eq!(
            result.get("JDBC_DATABASE_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432/testdb?user=AzureDiamond&password=hunter2&sslmode=require"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_USERNAME"),
            Some(&String::from("AzureDiamond"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_PASSWORD"),
            Some(&String::from("hunter2"))
        );

        assert_eq!(result.get("SPRING_DATASOURCE_URL"), None);
        assert_eq!(result.get("SPRING_DATASOURCE_USERNAME"), None);
        assert_eq!(result.get("SPRING_DATASOURCE_PASSWORD"), None);
    }

    #[test]
    fn custom_database_url_without_password_and_path() {
        let result = jvm_env_vars_for_env(&HashMap::from([(
            String::from("DATABASE_URL"),
            String::from("postgres://test123@ec2-52-13-12"),
        )]))
        .unwrap();

        assert_eq!(result, HashMap::new());
    }

    #[test]
    fn custom_database_url_with_fragment_and_query_parameters() {
        let result =  jvm_env_vars_for_env(&HashMap::from([(
            String::from("DATABASE_URL"),
            String::from("postgres://AzureDiamond:hunter2@db.example.com:5432/testdb?foo=bar&e=mc^2#fragment")
        ),])).unwrap();

        assert_eq!(
            result.get("JDBC_DATABASE_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432/testdb?foo=bar&e=mc^2&user=AzureDiamond&password=hunter2&sslmode=require#fragment"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_USERNAME"),
            Some(&String::from("AzureDiamond"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_PASSWORD"),
            Some(&String::from("hunter2"))
        );
    }
    #[test]
    fn custom_database_url_without_path() {
        let result = jvm_env_vars_for_env(&HashMap::from([(
            String::from("DATABASE_URL"),
            String::from("postgres://AzureDiamond:hunter2@db.example.com:5432"),
        )]))
        .unwrap();

        assert_eq!(
            result.get("JDBC_DATABASE_URL"),
            Some(&String::from("jdbc:postgresql://db.example.com:5432?user=AzureDiamond&password=hunter2&sslmode=require"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_USERNAME"),
            Some(&String::from("AzureDiamond"))
        );

        assert_eq!(
            result.get("JDBC_DATABASE_PASSWORD"),
            Some(&String::from("hunter2"))
        );
    }

    #[test]
    fn spring_redis_url() {
        let result = jvm_env_vars_for_env(&HashMap::from([(
            String::from("REDIS_URL"),
            String::from("redis://h:asdfqwer1234asdf@ec2-111-1-1-1.compute-1.amazonaws.com:111"),
        )]))
        .unwrap();

        assert_eq!(
            result.get("SPRING_REDIS_URL"),
            Some(&String::from(
                "redis://h:asdfqwer1234asdf@ec2-111-1-1-1.compute-1.amazonaws.com:111"
            ))
        );
    }

    #[test]
    fn spring_redis_url_disabled() {
        let result = jvm_env_vars_for_env(&HashMap::from([
            (
                String::from("REDIS_URL"),
                String::from(
                    "redis://h:asdfqwer1234asdf@ec2-111-1-1-1.compute-1.amazonaws.com:111",
                ),
            ),
            (
                String::from("DISABLE_SPRING_REDIS_URL"),
                String::from("true"),
            ),
        ]))
        .unwrap();

        assert_eq!(result.get("SPRING_REDIS_URL"), None);
    }

    #[test]
    fn spring_redis_url_already_set() {
        let result = jvm_env_vars_for_env(&HashMap::from([
            (
                String::from("REDIS_URL"),
                String::from(
                    "redis://h:asdfqwer1234asdf@ec2-111-1-1-1.compute-1.amazonaws.com:111",
                ),
            ),
            (
                String::from("SPRING_REDIS_URL"),
                String::from(
                    "redis://h:asdfqwer1234asdf@ec2-111-1-1-1.compute-1.amazonaws.com:222",
                ),
            ),
        ]))
        .unwrap();

        assert_eq!(result.get("SPRING_REDIS_URL"), None);
    }
}
