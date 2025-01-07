// cargo-llvm-cov sets the coverage_nightly attribute when instrumenting our code. In that case,
// we enable https://doc.rust-lang.org/beta/unstable-book/language-features/coverage-attribute.html
// to be able selectively opt out of coverage for functions/lines/modules.
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod configuration;
mod detect;
mod errors;
mod layers;
mod sbt;

use crate::configuration::read_sbt_buildpack_configuration;
use crate::detect::is_sbt_project_directory;
use crate::errors::{log_user_errors, SbtBuildpackError};
use crate::layers::dependency_resolver_home::{
    handle_dependency_resolver_home, DependencyResolver,
};
use crate::layers::sbt_boot::handle_sbt_boot;
use crate::layers::sbt_extras::handle_sbt_extras;
use crate::layers::sbt_global::handle_sbt_global;
use buildpacks_jvm_shared::system_properties::read_system_properties;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::{buildpack_main, Buildpack, Env, Error, Platform};
use libherokubuildpack::command::CommandExt;
use libherokubuildpack::error::on_error as on_buildpack_error;
use libherokubuildpack::log::{log_header, log_info};
use std::io::{stderr, stdout};
use std::process::Command;

#[cfg(test)]
use buildpacks_jvm_shared_test as _;
#[cfg(test)]
use libcnb_test as _;
#[cfg(test)]
use tempfile as _;
#[cfg(test)]
use ureq as _;

struct SbtBuildpack;

impl Buildpack for SbtBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = SbtBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        let is_sbt_project = is_sbt_project_directory(&context.app_dir)
            .map_err(SbtBuildpackError::DetectPhaseIoError)?;

        if is_sbt_project {
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
        let buildpack_configuration = read_system_properties(&context.app_dir)
            .map_err(SbtBuildpackError::ReadSystemPropertiesError)
            .and_then(|system_properties| {
                read_sbt_buildpack_configuration(&system_properties, context.platform.env())
                    .map_err(SbtBuildpackError::ReadSbtBuildpackConfigurationError)
            })?;

        let sbt_version = sbt::version::read_sbt_version(&context.app_dir)
            .map_err(SbtBuildpackError::ReadSbtVersionError)
            .and_then(|version| version.ok_or(SbtBuildpackError::UnknownSbtVersion))?;

        if !sbt::version::is_supported_sbt_version(&sbt_version) {
            Err(SbtBuildpackError::UnsupportedSbtVersion(
                sbt_version.clone(),
            ))?;
        }

        let sbt_available_at_launch = buildpack_configuration
            .sbt_available_at_launch
            .unwrap_or_default();

        let mut env = Env::from_current();

        handle_dependency_resolver_home(
            &context,
            sbt_available_at_launch,
            DependencyResolver::Ivy,
            &mut env,
        )?;

        handle_dependency_resolver_home(
            &context,
            sbt_available_at_launch,
            DependencyResolver::Coursier,
            &mut env,
        )?;

        handle_sbt_extras(&context, sbt_available_at_launch, &mut env)?;
        handle_sbt_boot(&context, sbt_version, sbt_available_at_launch, &mut env)?;
        handle_sbt_global(&context, sbt_available_at_launch, &mut env)?;

        log_header("Building Scala project");

        let tasks = sbt::tasks::from_config(&buildpack_configuration);
        log_info(format!("Running: sbt {}", shell_words::join(&tasks)));

        let output = Command::new("sbt")
            .current_dir(&context.app_dir)
            .args(tasks)
            .envs(&env)
            .output_and_write_streams(stdout(), stderr())
            .map_err(SbtBuildpackError::SbtBuildIoError)?;

        output.status.success().then_some(()).ok_or(
            SbtBuildpackError::SbtBuildUnexpectedExitStatus(
                output.status,
                sbt::output::parse_errors(&output.stdout),
            ),
        )?;

        BuildResultBuilder::new().build()
    }

    fn on_error(&self, error: Error<Self::Error>) {
        on_buildpack_error(log_user_errors, error);
    }
}

buildpack_main!(SbtBuildpack);
