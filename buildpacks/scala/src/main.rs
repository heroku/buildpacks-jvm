// Enable rustc and Clippy lints that are disabled by default.
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

mod build_configuration;
mod detection;
mod errors;
mod layers;
mod paths;

use crate::build_configuration::{create_build_config, BuildConfiguration};
use crate::detection::detect_sbt;
use crate::errors::ScalaBuildpackError::{SbtBuildIoError, SbtBuildUnexpectedExitCode};
use crate::errors::{log_user_errors, ScalaBuildpackError};
use crate::layers::ivy_cache::IvyCacheLayer;
use crate::layers::sbt::SbtLayer;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::{BuildPlan, BuildPlanBuilder};
use libcnb::data::layer_name;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::layer_env::Scope;
use libcnb::{buildpack_main, Buildpack, Env, Error, Platform};
use libherokubuildpack::error::on_error as on_buildpack_error;
use libherokubuildpack::log::log_header;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

pub struct ScalaBuildpack;

impl Buildpack for ScalaBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = ScalaBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        if !detect_sbt(&context.app_dir) {
            return DetectResultBuilder::fail().build();
        }

        DetectResultBuilder::pass()
            .build_plan(create_scala_build_plan())
            .build()
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let env = Env::from_current();
        let build_config = create_build_config(&context.app_dir, context.platform.env())?;

        let env = create_ivy_cache_layer(&context, &env)?;
        let env = create_sbt_layer(&context, &env, &build_config)?;

        run_sbt_tasks(&context.app_dir, &build_config, &env)?;

        BuildResultBuilder::new().build()
    }

    fn on_error(&self, error: Error<Self::Error>) {
        on_buildpack_error(log_user_errors, error);
    }
}

buildpack_main!(ScalaBuildpack);

fn create_scala_build_plan() -> BuildPlan {
    BuildPlanBuilder::new()
        .requires("jdk")
        .provides("sbt")
        .requires("sbt")
        .build()
}

fn create_ivy_cache_layer(
    context: &BuildContext<ScalaBuildpack>,
    env: &Env,
) -> Result<Env, Error<ScalaBuildpackError>> {
    let ivy_cache_layer = context.handle_layer(layer_name!("ivy_cache"), IvyCacheLayer)?;
    Ok(ivy_cache_layer.env.apply(Scope::Build, env))
}

fn create_sbt_layer(
    context: &BuildContext<ScalaBuildpack>,
    env: &Env,
    build_config: &BuildConfiguration,
) -> Result<Env, Error<ScalaBuildpackError>> {
    log_header("Installing sbt");
    let sbt_home_layer = context.handle_layer(
        layer_name!("sbt"),
        SbtLayer {
            sbt_version: build_config.sbt_version.clone(),
            env: env.clone(),
        },
    )?;
    Ok(sbt_home_layer.env.apply(Scope::Build, env))
}

fn run_sbt_tasks(
    app_dir: &PathBuf,
    _build_config: &BuildConfiguration,
    env: &Env,
) -> Result<ExitStatus, ScalaBuildpackError> {
    log_header("Building Scala project");

    Command::new("sbt-extras")
        .current_dir(app_dir)
        .args(["compile", "stage"])
        .envs(env)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(SbtBuildIoError)
        .and_then(|exit_status| {
            if exit_status.success() {
                Ok(exit_status)
            } else {
                Err(SbtBuildUnexpectedExitCode(exit_status))
            }
        })
}
