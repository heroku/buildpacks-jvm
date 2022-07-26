use libcnb::data::buildpack::StackId;
use std::fs::File;
use std::path::Path;

pub fn normalize_version_string<S: Into<String>>(
    stack_id: &StackId,
    user_version_string: S,
) -> Result<(OpenJDKDistribution, String), NormalizeVersionStringError> {
    let user_version_string = user_version_string.into();

    let (user_distribution_string, user_version_string) = user_version_string
        .trim()
        .split_once("-")
        .map(|(split_distribution_string, split_version_string)| {
            (Some(split_distribution_string), split_version_string)
        })
        .unwrap_or((None, &user_version_string));

    let version_string = match user_version_string {
        "7" | "1.7" => "1.7.0_352",
        "8" | "1.8" => "1.8.0_342",
        "9" | "1.9" => "9.0.4",
        "10" => "10.0.2",
        "11" => "11.0.16",
        "12" => "12.0.2",
        "13" => "13.0.12",
        "14" => "14.0.2",
        "15" => "15.0.8",
        "16" => "16.0.2",
        "17" => "17.0.4",
        "18" => "18.0.2",
        other => other,
    };

    match user_distribution_string {
        None => Ok(default_distribution(&stack_id)),
        Some("heroku") | Some("openjdk") => Ok(OpenJDKDistribution::Heroku),
        Some("zulu") => Ok(OpenJDKDistribution::AzulZulu),
        Some(unknown) => Err(NormalizeVersionStringError::UnknownDistribution(
            String::from(unknown),
        )),
    }
    .map(|distribution| (distribution, String::from(version_string)))
}

fn default_distribution(stack_id: &StackId) -> OpenJDKDistribution {
    match stack_id.as_str() {
        "heroku-18" | "heroku-20" => OpenJDKDistribution::Heroku,
        _ => OpenJDKDistribution::AzulZulu,
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum NormalizeVersionStringError {
    UnknownDistribution(String),
}

pub fn resolve_openjdk_url<V: Into<String>>(
    stack_id: &StackId,
    distribution: OpenJDKDistribution,
    version_string: V,
) -> String {
    let version_string = version_string.into();
    let base_url = format!("https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/{stack_id}");

    let file_name = match distribution {
        OpenJDKDistribution::Heroku => format!("openjdk{version_string}.tar.gz"),
        OpenJDKDistribution::AzulZulu => format!("zulu-{version_string}.tar.gz"),
    };

    format!("{base_url}/{file_name}")
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpenJDKDistribution {
    Heroku,
    AzulZulu,
}

pub fn read_version_string_from_app_dir<P: AsRef<Path>>(
    app_dir: P,
) -> Result<Option<String>, ReadVersionStringError> {
    let system_properties_path = app_dir.as_ref().join("system.properties");

    if system_properties_path.exists() {
        File::open(&system_properties_path)
            .map_err(ReadVersionStringError::CannotReadSystemProperties)
            .and_then(|file| {
                java_properties::read(&file).map_err(ReadVersionStringError::InvalidPropertiesFile)
            })
            .map(|properties| properties.get("java.runtime.version").cloned())
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub enum ReadVersionStringError {
    CannotReadSystemProperties(std::io::Error),
    InvalidPropertiesFile(java_properties::PropertiesError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use libcnb::data::stack_id;

    #[test]
    fn normalize_version_string_stack_specific_distribution() {
        assert_eq!(
            normalize_version_string(&stack_id!("heroku-18"), "8"),
            Ok((OpenJDKDistribution::Heroku, String::from("1.8.0_342")))
        );

        assert_eq!(
            normalize_version_string(&stack_id!("heroku-20"), "8"),
            Ok((OpenJDKDistribution::Heroku, String::from("1.8.0_342")))
        );

        assert_eq!(
            normalize_version_string(&stack_id!("heroku-22"), "8"),
            Ok((OpenJDKDistribution::AzulZulu, String::from("1.8.0_342")))
        );

        assert_eq!(
            normalize_version_string(&stack_id!("bogus"), "8"),
            Ok((OpenJDKDistribution::AzulZulu, String::from("1.8.0_342")))
        );
    }

    #[test]
    fn foo() {
        assert_eq!(
            resolve_openjdk_url(
                &stack_id!("heroku-20"),
                OpenJDKDistribution::Heroku,
                "1.0.0"
            ),
            "https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-20/openjdk1.0.0.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                &stack_id!("heroku-20"),
                OpenJDKDistribution::Heroku,
                "1.2.3"
            ),
            "https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-20/openjdk1.2.3.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                &stack_id!("heroku-22"),
                OpenJDKDistribution::Heroku,
                "1.2.3"
            ),
            "https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-22/openjdk1.2.3.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                &stack_id!("heroku-18"),
                OpenJDKDistribution::Heroku,
                "1.2.3.4.5-suffix"
            ),
            "https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-18/openjdk1.2.3.4.5-suffix.tar.gz"
        );
    }

    #[test]
    fn foo_zulu() {
        assert_eq!(
            resolve_openjdk_url(
                &stack_id!("heroku-20"),
                OpenJDKDistribution::AzulZulu,
                "1.0.0"
            ),
            "https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-20/zulu-1.0.0.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                &stack_id!("heroku-20"),
                OpenJDKDistribution::AzulZulu,
                "1.2.3"
            ),
            "https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-20/zulu-1.2.3.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                &stack_id!("heroku-22"),
                OpenJDKDistribution::AzulZulu,
                "1.2.3"
            ),
            "https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-22/zulu-1.2.3.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                &stack_id!("heroku-18"),
                OpenJDKDistribution::AzulZulu,
                "1.2.3.4.5-suffix"
            ),
            "https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-18/zulu-1.2.3.4.5-suffix.tar.gz"
        );
    }
}
