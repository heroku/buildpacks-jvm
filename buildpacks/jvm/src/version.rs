pub(crate) fn normalize_version_string<S: Into<String>>(
    user_version_string: S,
) -> Result<(OpenJDKDistribution, String), NormalizeVersionStringError> {
    let user_version_string = user_version_string.into();

    let (user_distribution_string, user_version_string) =
        user_version_string.trim().split_once('-').map_or(
            (None, user_version_string.as_str()),
            |(split_distribution_string, split_version_string)| {
                (Some(split_distribution_string), split_version_string)
            },
        );

    let version_string = match user_version_string {
        "7" | "1.7" => "1.7.0_352",
        "8" | "1.8" => "1.8.0_412",
        "9" | "1.9" => "9.0.4",
        "10" => "10.0.2",
        "11" => "11.0.23",
        "12" => "12.0.2",
        "13" => "13.0.14",
        "14" => "14.0.2",
        "15" => "15.0.10",
        "16" => "16.0.2",
        "17" => "17.0.11",
        "18" => "18.0.2.1",
        "19" => "19.0.2",
        "20" => "20.0.2",
        "21" => "21.0.3",
        other => other,
    };

    match user_distribution_string {
        None => Ok(OpenJDKDistribution::default()),
        Some("zulu") => Ok(OpenJDKDistribution::AzulZulu),
        Some(unknown) => Err(NormalizeVersionStringError::UnknownDistribution(
            String::from(unknown),
        )),
    }
    .map(|distribution| (distribution, String::from(version_string)))
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum NormalizeVersionStringError {
    UnknownDistribution(String),
}

pub(crate) fn resolve_openjdk_url<V: Into<String>>(
    distribution: OpenJDKDistribution,
    version_string: V,
) -> String {
    let version_string = version_string.into();

    match distribution {
        // We're using the legacy stack specific URL of heroku-22 for all targets. This is not a
        // problem as the distribution hosted there is NOT stack specific. This will eventually be
        // replaced with a stack agnostic URL.
        OpenJDKDistribution::AzulZulu => format!("https://lang-jvm.s3.us-east-1.amazonaws.com/jdk/heroku-22/zulu-{version_string}.tar.gz"),
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub(crate) enum OpenJDKDistribution {
    #[default]
    AzulZulu,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_version_string() {
        let zulu = OpenJDKDistribution::AzulZulu;

        let latest_java_8 = "1.8.0_412";
        let latest_java_11 = "11.0.23";
        let latest_java_21 = "21.0.3";

        let test_cases = [
            // OpenJDK 8
            ("8", Ok((zulu, String::from(latest_java_8)))),
            (latest_java_8, Ok((zulu, String::from(latest_java_8)))),
            ("zulu-8", Ok((zulu, String::from(latest_java_8)))),
            (
                &format!("zulu-{latest_java_8}"),
                Ok((zulu, String::from(latest_java_8))),
            ),
            // OpenJDK 11
            ("11", Ok((zulu, String::from(latest_java_11)))),
            (latest_java_11, Ok((zulu, String::from(latest_java_11)))),
            ("zulu-11", Ok((zulu, String::from(latest_java_11)))),
            (
                &format!("zulu-{latest_java_11}"),
                Ok((zulu, String::from(latest_java_11))),
            ),
            // OpenJDK 21
            ("21", Ok((zulu, String::from(latest_java_21)))),
            ("zulu-21", Ok((zulu, String::from(latest_java_21)))),
            // Other
            ("1337", Ok((zulu, String::from("1337")))),
            (
                "4.8.15.16.23.42",
                Ok((zulu, String::from("4.8.15.16.23.42"))),
            ),
            // Errors
            (
                "heroku-21",
                Err(NormalizeVersionStringError::UnknownDistribution(
                    String::from("heroku"),
                )),
            ),
        ];

        for (input, expected_output) in test_cases {
            assert_eq!(normalize_version_string(input), expected_output);
        }
    }
}
