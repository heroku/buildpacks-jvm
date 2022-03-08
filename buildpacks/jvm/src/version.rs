use libcnb::data::buildpack::StackId;

pub fn normalize_version_string<S: Into<String>>(
    user_version_string: S,
) -> Result<(OpenJDKDistribution, String), NormalizeVersionStringError> {
    let user_version_string = user_version_string.into();

    let (user_distribution_string, user_version_string) = user_version_string
        .split_once("-")
        .unwrap_or(("heroku", &user_version_string));

    let version_string = match user_version_string {
        "7" | "1.7" => "1.7.0_332",
        "8" | "1.8" => "1.8.0_322",
        "9" | "1.9" => "9.0.4",
        "10" => "10.0.2",
        "11" => "11.0.14.1",
        "12" => "12.0.2",
        "13" => "13.0.10",
        "14" => "14.0.2",
        "15" => "15.0.6",
        "16" => "16.0.2",
        "17" => "17.0.2",
        other => other,
    };

    match user_distribution_string {
        "heroku" | "openjdk" => Ok(OpenJDKDistribution::Heroku),
        "zulu" => Ok(OpenJDKDistribution::AzulZulu),
        unknown => Err(NormalizeVersionStringError::UnknownDistribution(
            String::from(unknown),
        )),
    }
    .map(|distribution| (distribution, String::from(version_string)))
}

#[derive(Debug)]
pub enum NormalizeVersionStringError {
    UnknownDistribution(String),
}

pub fn resolve_openjdk_url<V: Into<String>>(
    stack_id: StackId,
    distribution: OpenJDKDistribution,
    version_string: V,
) -> String {
    let version_string = version_string.into();
    let base_url = format!("https://lang-jvm.s3.amazonaws.com/jdk/{stack_id}");

    let file_name = match distribution {
        OpenJDKDistribution::Heroku => format!("openjdk{version_string}.tar.gz"),
        OpenJDKDistribution::AzulZulu => format!("zulu-{version_string}.tar.gz"),
    };

    format!("{base_url}/{file_name}")
}

#[derive(Debug)]
pub enum OpenJDKDistribution {
    Heroku,
    AzulZulu,
}

#[cfg(test)]
mod tests {
    use super::*;
    use libcnb::data::stack_id;

    #[test]
    fn foo() {
        assert_eq!(
            resolve_openjdk_url(stack_id!("heroku-20"), OpenJDKDistribution::Heroku, "1.0.0"),
            "https://lang-jvm.s3.amazonaws.com/jdk/heroku-20/openjdk1.0.0.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(stack_id!("heroku-20"), OpenJDKDistribution::Heroku, "1.2.3"),
            "https://lang-jvm.s3.amazonaws.com/jdk/heroku-20/openjdk1.2.3.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(stack_id!("heroku-22"), OpenJDKDistribution::Heroku, "1.2.3"),
            "https://lang-jvm.s3.amazonaws.com/jdk/heroku-22/openjdk1.2.3.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                stack_id!("heroku-18"),
                OpenJDKDistribution::Heroku,
                "1.2.3.4.5-suffix"
            ),
            "https://lang-jvm.s3.amazonaws.com/jdk/heroku-18/openjdk1.2.3.4.5-suffix.tar.gz"
        );
    }

    #[test]
    fn foo_zulu() {
        assert_eq!(
            resolve_openjdk_url(
                stack_id!("heroku-20"),
                OpenJDKDistribution::AzulZulu,
                "1.0.0"
            ),
            "https://lang-jvm.s3.amazonaws.com/jdk/heroku-20/zulu-1.0.0.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                stack_id!("heroku-20"),
                OpenJDKDistribution::AzulZulu,
                "1.2.3"
            ),
            "https://lang-jvm.s3.amazonaws.com/jdk/heroku-20/zulu-1.2.3.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                stack_id!("heroku-22"),
                OpenJDKDistribution::AzulZulu,
                "1.2.3"
            ),
            "https://lang-jvm.s3.amazonaws.com/jdk/heroku-22/zulu-1.2.3.tar.gz"
        );

        assert_eq!(
            resolve_openjdk_url(
                stack_id!("heroku-18"),
                OpenJDKDistribution::AzulZulu,
                "1.2.3.4.5-suffix"
            ),
            "https://lang-jvm.s3.amazonaws.com/jdk/heroku-18/zulu-1.2.3.4.5-suffix.tar.gz"
        );
    }
}
