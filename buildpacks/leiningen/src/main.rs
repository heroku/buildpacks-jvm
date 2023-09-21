// Enable rustc and Clippy lints that are disabled by default.
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

mod errors;

use crate::errors::{log_user_errors, LeiningenBuildpackError};
use buildpacks_jvm_shared::fs::set_executable;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::{buildpack_main, Buildpack, Env, Error};
use libherokubuildpack::error::on_error as on_buildpack_error;
use std::process::Command;

pub(crate) struct LeiningenBuildpack;

impl Buildpack for LeiningenBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = LeiningenBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        if context.app_dir.join("project.clj").exists() {
            DetectResultBuilder::pass()
                .build_plan(
                    BuildPlanBuilder::new()
                        .requires("jdk")
                        .provides("jvm-application")
                        .requires("jvm-application")
                        .build(),
                )
                .build()
        } else {
            DetectResultBuilder::fail().build()
        }
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let lein_contents = include_str!("../lein/lein");
        let lein_path = context.app_dir.join("lein");

        std::fs::write(&lein_path, lein_contents).unwrap();
        set_executable(&lein_path).unwrap();

        Command::new(lein_path)
            .arg("uberjar")
            .envs(&Env::from_current())
            .spawn()
            .unwrap();

        BuildResultBuilder::new().build()
    }

    fn on_error(&self, error: Error<Self::Error>) {
        on_buildpack_error(log_user_errors, error);
    }
}

buildpack_main!(LeiningenBuildpack);
