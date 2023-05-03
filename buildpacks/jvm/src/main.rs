// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

mod constants;
mod errors;
mod layers;
mod util;
mod version;

use crate::constants::SKIP_HEROKU_JVM_METRICS_AGENT_INSTALLATION_ENV_VAR_NAME;
use crate::errors::on_error_jvm_buildpack;
use crate::layers::heroku_metrics_agent::HerokuMetricsAgentLayer;
use crate::layers::openjdk::OpenJdkLayer;
use crate::layers::runtime::RuntimeLayer;
use crate::util::{boolean_buildpack_config_env_var, ValidateSha256Error};
use crate::version::NormalizeVersionStringError;
pub(crate) use constants::{
    JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER, JAVA_TOOL_OPTIONS_ENV_VAR_NAME, JDK_OVERLAY_DIR_NAME,
};
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::layer_name;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::Buildpack;
use libcnb::{buildpack_main, Platform};
#[cfg(test)]
use libcnb_test as _;
use libherokubuildpack::download::DownloadError;
use serde::{Deserialize, Serialize};
// Work around unused_crate_dependencies. url is used in heroku_database_env_var_rewrite.
use buildpacks_jvm_shared::system_properties::{read_system_properties, ReadSystemPropertiesError};
use url as _;

pub(crate) struct OpenJdkBuildpack;

#[derive(Debug)]
pub(crate) enum OpenJdkBuildpackError {
    OpenJdkDownloadError(DownloadError),
    MetricsAgentDownloadError(DownloadError),
    MetricsAgentSha256ValidationError(ValidateSha256Error),
    CannotCreateOpenJdkTempDir(std::io::Error),
    CannotOpenOpenJdkTarball(std::io::Error),
    CannotDecompressOpenJdkTarball(std::io::Error),
    ReadVersionStringError(ReadSystemPropertiesError),
    NormalizeVersionStringError(NormalizeVersionStringError),
    MissingJdkCertificatesFile,
    CannotSymlinkUbuntuCertificates(std::io::Error),
    CannotListJdkOverlayContents(std::io::Error),
    CannotCopyJdkOverlayContents(fs_extra::error::Error),
}

impl Buildpack for OpenJdkBuildpack {
    type Platform = GenericPlatform;
    type Metadata = OpenJdkBuildpackMetadata;
    type Error = OpenJdkBuildpackError;

    fn detect(&self, _context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        DetectResultBuilder::pass()
            .build_plan(
                BuildPlanBuilder::new()
                    .provides("jdk")
                    .requires("jdk")
                    .build(),
            )
            .build()
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let app_dir_version_string = read_system_properties(&context.app_dir)
            .map(|properties| properties.get("java.runtime.version").cloned())
            .map_err(OpenJdkBuildpackError::ReadVersionStringError)?;

        let normalized_version = version::normalize_version_string(
            &context.stack_id,
            app_dir_version_string.unwrap_or_else(|| String::from("8")),
        )
        .map_err(OpenJdkBuildpackError::NormalizeVersionStringError)?;

        context.handle_layer(
            layer_name!("openjdk"),
            OpenJdkLayer {
                tarball_url: version::resolve_openjdk_url(
                    &context.stack_id,
                    normalized_version.0,
                    normalized_version.1,
                ),
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
pub(crate) struct OpenJdkBuildpackMetadata {
    #[serde(rename = "heroku-metrics-agent")]
    heroku_metrics_agent: HerokuMetricsAgentMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub(crate) struct HerokuMetricsAgentMetadata {
    url: String,
    sha256: String,
}

buildpack_main!(OpenJdkBuildpack);

impl From<OpenJdkBuildpackError> for libcnb::Error<OpenJdkBuildpackError> {
    fn from(error: OpenJdkBuildpackError) -> Self {
        libcnb::Error::BuildpackError(error)
    }
}
