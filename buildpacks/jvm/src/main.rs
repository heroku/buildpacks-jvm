// Enable rustc and Clippy lints that are disabled by default.
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

mod constants;
mod layers;
mod util;
mod version;

use crate::layers::heroku_metrics_agent::HerokuMetricsAgentLayer;
use crate::layers::openjdk::OpenJdkLayer;
use crate::layers::runtime::RuntimeLayer;
use crate::util::ValidateSha256Error;
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
use std::fs::File;
use std::path::Path;

pub struct OpenJdkBuildpack;

#[derive(Debug)]
pub enum OpenJdkBuildpackError {
    OpenJdkDownloadError(DownloadError),
    MetricsAgentDownloadError(DownloadError),
    MetricsAgentSha256ValidationError(ValidateSha256Error),
    CannotCreateTempDir(std::io::Error),
    CannotOpenOpenJdkTarball(std::io::Error),
    CannotDecompressOpenJdkTarball(std::io::Error),
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
        let x = match read_version_string_from_app_dir(&context.app_dir) {
            Ok(Some(user_version)) => version::normalize_version_string(user_version),
            Ok(None) => version::normalize_version_string("8"),
            Err(ReadVersionStringError::CannotReadSystemProperties(_)) => {
                version::normalize_version_string("8")
            }
            Err(ReadVersionStringError::InvalidPropertiesFile(_)) => {
                version::normalize_version_string("8")
            }
        }
        .unwrap();

        context.handle_layer(
            layer_name!("openjdk"),
            OpenJdkLayer {
                tarball_url: version::resolve_openjdk_url(&context.stack_id, x.0, x.1),
            },
        )?;

        context.handle_layer(layer_name!("heroku_metrics_agent"), HerokuMetricsAgentLayer)?;
        context.handle_layer(layer_name!("runtime"), RuntimeLayer)?;

        BuildResultBuilder::new().build()
    }
}

fn read_version_string_from_app_dir<P: AsRef<Path>>(
    app_dir: P,
) -> Result<Option<String>, ReadVersionStringError> {
    File::open(app_dir.as_ref().join("system.properties"))
        .map_err(ReadVersionStringError::CannotReadSystemProperties)
        .and_then(|file| {
            java_properties::read(&file).map_err(ReadVersionStringError::InvalidPropertiesFile)
        })
        .map(|properties| properties.get("java.runtime.version").cloned())
}

#[derive(Debug)]
enum ReadVersionStringError {
    CannotReadSystemProperties(std::io::Error),
    InvalidPropertiesFile(java_properties::PropertiesError),
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
