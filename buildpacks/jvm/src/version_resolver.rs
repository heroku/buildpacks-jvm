use crate::constants::OPENJDK_LATEST_LTS_VERSION;
use crate::openjdk_artifact::{
    HerokuOpenJdkVersionRequirement, OpenJdkArtifactRequirement,
    OpenJdkArtifactRequirementParseError, OpenJdkDistribution,
};
use crate::salesforce_functions::is_salesforce_function_app;
use buildpacks_jvm_shared::system_properties::{ReadSystemPropertiesError, read_system_properties};
use std::path::Path;

pub(crate) fn resolve_version(app_dir: &Path) -> Result<ResolveResult, VersionResolveError> {
    let openjdk_artifact_requirement = read_system_properties(app_dir)
        .map_err(VersionResolveError::ReadSystemPropertiesError)
        .map(|properties| properties.get("java.runtime.version").cloned())
        .and_then(|string| {
            string
                .map(|string| {
                    string
                        .parse::<OpenJdkArtifactRequirement>()
                        .map_err(VersionResolveError::OpenJdkArtifactRequirementParseError)
                })
                .transpose()
        })?;

    let result = match openjdk_artifact_requirement {
        // The default version for Salesforce functions is always OpenJDK 8. Keep this conditional
        // around until Salesforce functions is EOL and then remove it.
        None if is_salesforce_function_app(app_dir) => ResolveResult {
            source: OpenJdkArtifactRequirementSource::DefaultVersionFunctions,
            requirement: OpenJdkArtifactRequirement {
                version: HerokuOpenJdkVersionRequirement::Major(8),
                distribution: OpenJdkDistribution::default(),
            },
        },
        None => ResolveResult {
            source: OpenJdkArtifactRequirementSource::DefaultVersionLatestLts,
            requirement: OpenJdkArtifactRequirement {
                version: HerokuOpenJdkVersionRequirement::Major(OPENJDK_LATEST_LTS_VERSION),
                distribution: OpenJdkDistribution::default(),
            },
        },
        Some(requirement) => ResolveResult {
            source: OpenJdkArtifactRequirementSource::SystemProperties,
            requirement,
        },
    };

    Ok(result)
}

pub(crate) struct ResolveResult {
    pub(crate) requirement: OpenJdkArtifactRequirement,
    pub(crate) source: OpenJdkArtifactRequirementSource,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum VersionResolveError {
    #[error("{0:?}")]
    ReadSystemPropertiesError(ReadSystemPropertiesError),
    #[error("{0:?}")]
    OpenJdkArtifactRequirementParseError(OpenJdkArtifactRequirementParseError),
}

#[derive(Eq, PartialEq)]
pub(crate) enum OpenJdkArtifactRequirementSource {
    SystemProperties,
    DefaultVersionLatestLts,
    DefaultVersionFunctions,
}
