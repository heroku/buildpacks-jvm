// Enable rustc and Clippy lints that are disabled by default.
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

mod constants;
mod errors;
mod layers;
mod util;
mod version;

use crate::errors::on_error_jvm_buildpack;
use crate::layers::heroku_metrics_agent::HerokuMetricsAgentLayer;
use crate::layers::openjdk::OpenJdkLayer;
use crate::layers::runtime::RuntimeLayer;
use crate::util::ValidateSha256Error;
use crate::version::{NormalizeVersionStringError, ReadVersionStringError};
pub use constants::*;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::buildpack_main;
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::layer_name;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::Buildpack;
use libherokubuildpack::DownloadError;
use serde::{Deserialize, Serialize};

pub struct OpenJdkBuildpack;

#[derive(Debug)]
pub enum OpenJdkBuildpackError {
    OpenJdkDownloadError(DownloadError),
    MetricsAgentDownloadError(DownloadError),
    MetricsAgentSha256ValidationError(ValidateSha256Error),
    CannotCreateOpenJdkTempDir(std::io::Error),
    CannotOpenOpenJdkTarball(std::io::Error),
    CannotDecompressOpenJdkTarball(std::io::Error),
    ReadVersionStringError(ReadVersionStringError),
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
        let app_dir_version_string = version::read_version_string_from_app_dir(&context.app_dir)
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

        context.handle_layer(layer_name!("heroku_metrics_agent"), HerokuMetricsAgentLayer)?;
        context.handle_layer(layer_name!("runtime"), RuntimeLayer)?;

        BuildResultBuilder::new().build()
    }

    fn on_error(&self, error: libcnb::Error<Self::Error>) {
        libherokubuildpack::on_error_heroku(on_error_jvm_buildpack, error);
    }
}

#[derive(Deserialize, Debug)]
pub struct OpenJdkBuildpackMetadata {
    #[serde(rename = "heroku-metrics-agent")]
    heroku_metrics_agent: HerokuMetricsAgentMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct HerokuMetricsAgentMetadata {
    url: String,
    sha256: String,
}

buildpack_main!(OpenJdkBuildpack);

impl From<OpenJdkBuildpackError> for libcnb::Error<OpenJdkBuildpackError> {
    fn from(error: OpenJdkBuildpackError) -> Self {
        libcnb::Error::BuildpackError(error)
    }
}
