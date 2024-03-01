mod constants;
mod errors;
mod layers;
mod util;
mod version;

use crate::constants::SKIP_HEROKU_JVM_METRICS_AGENT_INSTALLATION_ENV_VAR_NAME;
use crate::errors::on_error_jvm_buildpack;
use crate::util::{boolean_buildpack_config_env_var, ValidateSha256Error};
use crate::version::NormalizeVersionStringError;
use buildpacks_jvm_shared::system_properties::{read_system_properties, ReadSystemPropertiesError};
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::{buildpack_main, Platform};
use libcnb::{layer, Buildpack};
use libherokubuildpack::download::DownloadError;
use serde::{Deserialize, Serialize};
use url as _; // Used by exec.d binary

#[cfg(test)]
use buildpacks_jvm_shared_test as _;
#[cfg(test)]
use libcnb_test as _;

pub(crate) struct OpenJdkBuildpack;

#[derive(Debug)]
enum OpenJdkBuildpackError {
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
            .map_err(OpenJdkBuildpackError::ReadVersionStringError)?;

        let build_plan = if app_specifies_jvm_version {
            BuildPlanBuilder::new().provides("jdk").requires("jdk")
        } else {
            BuildPlanBuilder::new().provides("jdk")
        }
        .build();

        DetectResultBuilder::pass().build_plan(build_plan).build()
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let app_dir_version_string = read_system_properties(&context.app_dir)
            .map(|properties| properties.get("java.runtime.version").cloned())
            .map_err(OpenJdkBuildpackError::ReadVersionStringError)?;

        let normalized_version = version::normalize_version_string(
            FAKE_STACK_ID,
            app_dir_version_string.unwrap_or_else(|| String::from("8")),
        )
        .map_err(OpenJdkBuildpackError::NormalizeVersionStringError)?;

        layers::openjdk::handle(
            version::resolve_openjdk_url(FAKE_STACK_ID, normalized_version.0, normalized_version.1),
            &context,
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
            layers::heroku_metrics_agent::handle(&context)?;
        }

        layers::runtime::handle(&context)?;

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

const FAKE_STACK_ID: &str = "heroku-22";
