mod constants;
mod errors;
mod layers;
mod openjdk_artifact;
mod openjdk_version;
mod salesforce_functions;
mod util;

use crate::constants::OPENJDK_LATEST_LTS_VERSION;
use crate::errors::on_error_jvm_buildpack;
use crate::layers::openjdk::handle_openjdk_layer;
use crate::layers::runtime::handle_runtime_layer;
use crate::openjdk_artifact::{
    HerokuOpenJdkVersionRequirement, OpenJdkArtifactMetadata, OpenJdkArtifactRequirement,
    OpenJdkArtifactRequirementParseError, OpenJdkDistribution,
};
use crate::openjdk_version::OpenJdkVersion;
use crate::salesforce_functions::is_salesforce_function_app;
use buildpacks_jvm_shared::system_properties::{read_system_properties, ReadSystemPropertiesError};
#[cfg(test)]
use buildpacks_jvm_shared_test as _;
pub(crate) use constants::{
    JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER, JAVA_TOOL_OPTIONS_ENV_VAR_NAME, JDK_OVERLAY_DIR_NAME,
};
use indoc::formatdoc;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::buildpack_main;
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::Buildpack;
#[cfg(test)]
use libcnb_test as _;
use libherokubuildpack::download::DownloadError;
use libherokubuildpack::inventory::artifact::{Arch, Os};
use libherokubuildpack::inventory::{Inventory, ParseInventoryError};
use libherokubuildpack::log::log_warning;
use sha2::Sha256;
use std::env::consts;
use url as _; // Used by exec.d binary

struct OpenJdkBuildpack;

#[derive(Debug)]
enum OpenJdkBuildpackError {
    UnsupportedOpenJdkVersion(OpenJdkArtifactRequirement),
    OpenJdkDownloadError(DownloadError),
    CannotCreateOpenJdkTempDir(std::io::Error),
    CannotReadOpenJdkTarball(std::io::Error),
    OpenJdkTarballChecksumError { expected: Vec<u8>, actual: Vec<u8> },
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
    type Metadata = GenericMetadata;
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
            .map_err(OpenJdkBuildpackError::ReadSystemPropertiesError)
            .map(|properties| properties.get("java.runtime.version").cloned())
            .and_then(|string| {
                string
                    .map(|string| {
                        string
                            .parse::<OpenJdkArtifactRequirement>()
                            .map_err(OpenJdkBuildpackError::OpenJdkArtifactRequirementParseError)
                    })
                    .transpose()
            })?;

        let openjdk_artifact_requirement = if let Some(openjdk_artifact_requirement) =
            openjdk_artifact_requirement
        {
            openjdk_artifact_requirement
        // The default version for Salesforce functions is always OpenJDK 8. Keep this conditional
        // around until Salesforce functions is EOL and then remove it.
        } else if is_salesforce_function_app(&context.app_dir) {
            OpenJdkArtifactRequirement {
                version: HerokuOpenJdkVersionRequirement::Major(8),
                distribution: OpenJdkDistribution::default(),
            }
        } else {
            log_warning(
                "No OpenJDK version specified",
                formatdoc! {"
                    Your application does not explicitly specify an OpenJDK version. The latest
                    long-term support (LTS) version will be installed. This currently is OpenJDK {OPENJDK_LATEST_LTS_VERSION}.

                    This default version will change when a new LTS version is released. Your
                    application might fail to build with the new version. We recommend explicitly
                    setting the required OpenJDK version for your application.

                    To set the OpenJDK version, add or edit the system.properties file in the root
                    directory of your application to contain:

                    java.runtime.version = {OPENJDK_LATEST_LTS_VERSION}
                "},
            );

            OpenJdkArtifactRequirement {
                version: HerokuOpenJdkVersionRequirement::Major(OPENJDK_LATEST_LTS_VERSION),
                distribution: OpenJdkDistribution::default(),
            }
        };

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

        handle_openjdk_layer(&context, openjdk_artifact)?;
        handle_runtime_layer(&context)?;

        BuildResultBuilder::new().build()
    }

    fn on_error(&self, error: libcnb::Error<Self::Error>) {
        libherokubuildpack::error::on_error(on_error_jvm_buildpack, error);
    }
}

buildpack_main!(OpenJdkBuildpack);

impl From<OpenJdkBuildpackError> for libcnb::Error<OpenJdkBuildpackError> {
    fn from(error: OpenJdkBuildpackError) -> Self {
        libcnb::Error::BuildpackError(error)
    }
}
