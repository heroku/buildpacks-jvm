mod constants;
mod errors;
mod layers;
mod openjdk_artifact;
mod openjdk_version;
mod util;

use crate::constants::SKIP_HEROKU_JVM_METRICS_AGENT_INSTALLATION_ENV_VAR_NAME;
use crate::errors::on_error_jvm_buildpack;
use crate::layers::heroku_metrics_agent::HerokuMetricsAgentLayer;
use crate::layers::openjdk::OpenJdkLayer;
use crate::layers::runtime::RuntimeLayer;
use crate::openjdk_artifact::{
    OpenJdkArtifactMetadata, OpenJdkArtifactRequirement, OpenJdkArtifactRequirementParseError,
};
use crate::util::{boolean_buildpack_config_env_var, ValidateSha256Error};
use buildpacks_jvm_shared::system_properties::{read_system_properties, ReadSystemPropertiesError};
pub(crate) use constants::{
    JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER, JAVA_TOOL_OPTIONS_ENV_VAR_NAME, JDK_OVERLAY_DIR_NAME,
};
use inventory::artifact::{Arch, Os};
use inventory::inventory::{Inventory, ParseInventoryError};
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::layer_name;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::Buildpack;
use libcnb::{buildpack_main, Platform};
use libherokubuildpack::download::DownloadError;
use serde::{Deserialize, Serialize};
use std::env::consts;
use url as _; // Used by exec.d binary

use crate::openjdk_version::OpenJdkVersion;
#[cfg(test)]
use buildpacks_jvm_shared_test as _;
#[cfg(test)]
use libcnb_test as _;
use sha2::Sha256;

struct OpenJdkBuildpack;

#[derive(Debug)]
enum OpenJdkBuildpackError {
    UnsupportedOpenJdkVersion(OpenJdkArtifactRequirement),
    OpenJdkDownloadError(DownloadError),
    MetricsAgentDownloadError(DownloadError),
    MetricsAgentSha256ValidationError(ValidateSha256Error),
    CannotCreateOpenJdkTempDir(std::io::Error),
    CannotOpenOpenJdkTarball(std::io::Error),
    CannotDecompressOpenJdkTarball(std::io::Error),
    ReadSystemPropertiesError(ReadSystemPropertiesError),
    OpenJdkArtifactRequirementParseError(OpenJdkArtifactRequirementParseError),
    MissingJdkCertificatesFile,
    CannotSymlinkUbuntuCertificates(std::io::Error),
    CannotListJdkOverlayContents(std::io::Error),
    CannotCopyJdkOverlayContents(fs_extra::error::Error),
    ParseInventoryError(ParseInventoryError),
}

impl Buildpack for OpenJdkBuildpack {
    type Platform = GenericPlatform;
    type Metadata = OpenJdkBuildpackMetadata;
    type Error = OpenJdkBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        // This buildpack is first and foremost a buildpack that is designed for composing with
        // other buildpacks, usually with JVM build tools such as Maven or Gradle. To enable
        // other buildpacks to conditionally require the installation of OpenJDK, the detect of this
        // buildpack wil fail if no other buildpack requires "jdk".
        //
        // Some users might want to install OpenJDK without using another buildpack, which wouldn't
        // work with this buildpack since "jdk" would not be required in the build plan.
        // To enable this use-case, this buildpack will require "jdk" (itself) if the app contains
        // a system.properties file with a Java version. This is currently the way to define the
        // OpenJDK version on Heroku.
        let app_specifies_jvm_version = read_system_properties(&context.app_dir)
            .map(|properties| properties.contains_key("java.runtime.version"))
            .map_err(OpenJdkBuildpackError::ReadSystemPropertiesError)?;

        let build_plan = if app_specifies_jvm_version {
            BuildPlanBuilder::new().provides("jdk").requires("jdk")
        } else {
            BuildPlanBuilder::new().provides("jdk")
        }
        .build();

        DetectResultBuilder::pass().build_plan(build_plan).build()
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let openjdk_artifact_requirement = read_system_properties(&context.app_dir)
            .map(|properties| {
                properties
                    .get("java.runtime.version")
                    .cloned()
                    .unwrap_or(String::from("8"))
            })
            .map_err(OpenJdkBuildpackError::ReadSystemPropertiesError)
            .and_then(|string| {
                string
                    .parse::<OpenJdkArtifactRequirement>()
                    .map_err(OpenJdkBuildpackError::OpenJdkArtifactRequirementParseError)
            })?;

        let openjdk_inventory = include_str!("../openjdk_inventory.toml")
            .parse::<Inventory<OpenJdkVersion, Sha256, OpenJdkArtifactMetadata>>()
            .map_err(OpenJdkBuildpackError::ParseInventoryError)?;

        let openjdk_artifact = openjdk_inventory
            .partial_resolve(
                context
                    .target
                    .os
                    .parse::<Os>()
                    .expect("OS should be always parseable, buildpack will not run on unsupported operating systems."),
                // On platform API <= `0.9` together with lifecycle <= `0.17`, the `CNB_TARGET_ARCH` environment variable will not be set.
                // This will be the case for the `salesforce-functions` builder. To ensure this buildpack can run there, we will
                // fall back to Rust's architecture constant when the architecture cannot be determined. This workaround can be removed when
                // the `salesforce-functions` builder is EOL.
                Some(context.target.arch.as_str())
                    .filter(|value| !value.is_empty())
                    .unwrap_or(consts::ARCH)
                    .parse::<Arch>()
                    .expect("arch should be always parseable, buildpack will not run on unsupported architectures."),
                &openjdk_artifact_requirement,
            )
            .ok_or(OpenJdkBuildpackError::UnsupportedOpenJdkVersion(
                openjdk_artifact_requirement,
            ))?;

        context.handle_layer(
            layer_name!("openjdk"),
            OpenJdkLayer {
                artifact: openjdk_artifact,
            },
        )?;

        libherokubuildpack::log::log_header("Installing Heroku JVM metrics agent");

        if boolean_buildpack_config_env_var(
            context.platform.env(),
            SKIP_HEROKU_JVM_METRICS_AGENT_INSTALLATION_ENV_VAR_NAME,
        ) {
            libherokubuildpack::log::log_info(format!(
                "Skipping agent installation, {SKIP_HEROKU_JVM_METRICS_AGENT_INSTALLATION_ENV_VAR_NAME} environment variable is set to a truthy value."
            ));
        } else {
            context.handle_layer(layer_name!("heroku_metrics_agent"), HerokuMetricsAgentLayer)?;
        }

        context.handle_layer(layer_name!("runtime"), RuntimeLayer)?;

        BuildResultBuilder::new().build()
    }

    fn on_error(&self, error: libcnb::Error<Self::Error>) {
        libherokubuildpack::error::on_error(on_error_jvm_buildpack, error);
    }
}

#[derive(Deserialize, Debug)]
struct OpenJdkBuildpackMetadata {
    #[serde(rename = "heroku-metrics-agent")]
    heroku_metrics_agent: HerokuMetricsAgentMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
struct HerokuMetricsAgentMetadata {
    url: String,
    sha256: String,
}

buildpack_main!(OpenJdkBuildpack);

impl From<OpenJdkBuildpackError> for libcnb::Error<OpenJdkBuildpackError> {
    fn from(error: OpenJdkBuildpackError) -> Self {
        libcnb::Error::BuildpackError(error)
    }
}
