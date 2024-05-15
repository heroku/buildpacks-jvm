use crate::openjdk_version::OpenJdkVersion;
use inventory::version::{ArtifactRequirement, VersionRequirement};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct OpenJdkArtifactMetadata {
    pub(crate) distribution: OpenJdkDistribution,
    pub(crate) heroku_stack: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OpenJdkArtifactRequirement {
    pub(crate) version: HerokuOpenJdkVersionRequirement,
    pub(crate) distribution: OpenJdkDistribution,
}

impl FromStr for OpenJdkArtifactRequirement {
    type Err = OpenJdkArtifactRequirementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let user_version_string = String::from(s);

        let (user_distribution_string, user_version_string) =
            user_version_string.trim().split_once('-').map_or(
                (None, user_version_string.as_str()),
                |(split_distribution_string, split_version_string)| {
                    (Some(split_distribution_string), split_version_string)
                },
            );

        let version = user_version_string
            .parse::<u32>()
            .map(HerokuOpenJdkVersionRequirement::Major)
            .or_else(|_| {
                user_version_string
                    .parse::<OpenJdkVersion>()
                    .map(HerokuOpenJdkVersionRequirement::Specific)
                    .map_err(OpenJdkArtifactRequirementParseError::OpenJdkVersionParseError)
            })?;

        let distribution = match user_distribution_string {
            None => Ok(OpenJdkDistribution::default()),
            Some("zulu") => Ok(OpenJdkDistribution::AzulZulu),
            Some(unknown) => Err(OpenJdkArtifactRequirementParseError::UnknownDistribution(
                String::from(unknown),
            )),
        }?;

        Ok(OpenJdkArtifactRequirement {
            version,
            distribution,
        })
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum OpenJdkArtifactRequirementParseError {
    #[error("Unknown OpenJDK distribution '{0}'")]
    UnknownDistribution(String),
    #[error("OpenJDK version parse error: {0}")]
    OpenJdkVersionParseError(nom::error::Error<String>),
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum OpenJdkDistribution {
    #[default]
    #[serde(rename = "zulu")]
    AzulZulu,
}

impl ArtifactRequirement<OpenJdkVersion, OpenJdkArtifactMetadata> for OpenJdkArtifactRequirement {
    fn satisfies_metadata(&self, metadata: &OpenJdkArtifactMetadata) -> bool {
        metadata.distribution == self.distribution
    }

    fn satisfies_version(&self, version: &OpenJdkVersion) -> bool {
        self.version.satisfies(version)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum HerokuOpenJdkVersionRequirement {
    Major(u32),
    Specific(OpenJdkVersion),
}

impl VersionRequirement<OpenJdkVersion> for HerokuOpenJdkVersionRequirement {
    fn satisfies(&self, version: &OpenJdkVersion) -> bool {
        match self {
            HerokuOpenJdkVersionRequirement::Major(major_version) => {
                version.major() == *major_version
            }
            HerokuOpenJdkVersionRequirement::Specific(requested_version) => {
                version == requested_version
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requirement_from_string() {
        let zulu = OpenJdkDistribution::AzulZulu;

        for string in ["8", "zulu-8"] {
            assert_eq!(
                string.parse(),
                Ok(OpenJdkArtifactRequirement {
                    version: HerokuOpenJdkVersionRequirement::Major(8),
                    distribution: zulu,
                })
            );
        }

        for string in ["zulu-11.12.13", "zulu-11.12.13"] {
            assert_eq!(
                string.parse(),
                Ok(OpenJdkArtifactRequirement {
                    version: HerokuOpenJdkVersionRequirement::Specific("11.12.13".parse().unwrap()),
                    distribution: zulu,
                })
            );
        }

        assert_eq!(
            "thx-11.3.8".parse::<OpenJdkArtifactRequirement>(),
            Err(OpenJdkArtifactRequirementParseError::UnknownDistribution(
                String::from("thx")
            ))
        );

        assert_eq!(
            "lv-426#acheron".parse::<OpenJdkArtifactRequirement>(),
            Err(
                OpenJdkArtifactRequirementParseError::OpenJdkVersionParseError(
                    nom::error::Error::new(String::from("#acheron"), nom::error::ErrorKind::Eof)
                )
            )
        );
    }

    #[test]
    fn test_version_requirement_legacy() {
        let requirement = HerokuOpenJdkVersionRequirement::Major(8);

        assert!(requirement.satisfies(&"8u361".parse::<OpenJdkVersion>().unwrap()));
        assert!(requirement.satisfies(&"8u411".parse::<OpenJdkVersion>().unwrap()));
        assert!(requirement.satisfies(&"1.8.0_411".parse::<OpenJdkVersion>().unwrap()));

        assert!(!requirement.satisfies(&"22.0.1-ea".parse::<OpenJdkVersion>().unwrap()));
        assert!(!requirement.satisfies(&"9-ga+23".parse::<OpenJdkVersion>().unwrap()));
        assert!(!requirement.satisfies(&"7u351".parse::<OpenJdkVersion>().unwrap()));
    }

    #[test]
    fn test_version_requirement_jep_322() {
        let requirement = HerokuOpenJdkVersionRequirement::Major(11);

        assert!(requirement.satisfies(&"11-ga".parse::<OpenJdkVersion>().unwrap()));
        assert!(requirement.satisfies(&"11.0.1.2".parse::<OpenJdkVersion>().unwrap()));
        assert!(requirement.satisfies(&"11.0.0".parse::<OpenJdkVersion>().unwrap()));

        assert!(!requirement.satisfies(&"22.0.1".parse::<OpenJdkVersion>().unwrap()));
        assert!(!requirement.satisfies(&"8u361".parse::<OpenJdkVersion>().unwrap()));
        assert!(!requirement.satisfies(&"7u351".parse::<OpenJdkVersion>().unwrap()));
    }
}
