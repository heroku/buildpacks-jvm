// cargo-llvm-cov sets the coverage_nightly attribute when instrumenting our code. In that case,
// we enable https://doc.rust-lang.org/beta/unstable-book/language-features/coverage-attribute.html
// to be able selectively opt out of coverage for functions/lines/modules.
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use crate::config::GradleBuildpackConfig;
use crate::detect::is_gradle_project_directory;
use crate::errors::on_error_gradle_buildpack;
use crate::framework::{default_app_process, detect_framework, Framework};
use crate::gradle_command::GradleCommandError;
use crate::layers::gradle_home::handle_gradle_home_layer;
use crate::GradleBuildpackError::{GradleBuildIoError, GradleBuildUnexpectedStatusError};
use buildpacks_jvm_shared as shared;
use buildpacks_jvm_shared::output::{
    print_buildpack_name, print_section, print_subsection, track_buildpack_timing,
    track_subsection_timing, BuildpackOutputText, BuildpackOutputTextSection,
};
#[cfg(test)]
use buildpacks_jvm_shared_test as _;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::launch::LaunchBuilder;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::{buildpack_main, Buildpack, Env};
#[cfg(test)]
use libcnb_test as _;
use libherokubuildpack::command::CommandExt;
use serde::Deserialize;
use std::io::{stderr, stdout};
use std::process::{Command, ExitStatus};

mod config;
mod detect;
mod errors;
mod framework;
mod gradle_command;
mod layers;

struct GradleBuildpack;

#[derive(Debug)]
enum GradleBuildpackError {
    GradleWrapperNotFound,
    DetectError(std::io::Error),
    GradleBuildIoError(std::io::Error),
    GradleBuildUnexpectedStatusError(ExitStatus),
    GetTasksError(GradleCommandError<()>),
    GetDependencyReportError(GradleCommandError<()>),
    WriteGradlePropertiesError(std::io::Error),
    WriteGradleInitScriptError(std::io::Error),
    CannotSetGradleWrapperExecutableBit(std::io::Error),
    CannotDetermineDefaultAppProcess(std::io::Error),
    StartGradleDaemonError(GradleCommandError<()>),
    BuildTaskUnknown,
}

#[derive(Debug, Deserialize)]
struct GradleBuildpackMetadata {}

impl Buildpack for GradleBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GradleBuildpackMetadata;
    type Error = GradleBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        let is_gradle_project_directory = is_gradle_project_directory(&context.app_dir)
            .map_err(GradleBuildpackError::DetectError)?;

        if is_gradle_project_directory {
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
        track_buildpack_timing(|| {
            print_buildpack_name("Heroku Gradle Buildpack");

            let buildpack_config = GradleBuildpackConfig::from(&context);

            let gradle_wrapper_executable_path = Some(context.app_dir.join("gradlew"))
                .filter(|path| path.exists())
                .ok_or(GradleBuildpackError::GradleWrapperNotFound)?;

            shared::fs::set_executable(&gradle_wrapper_executable_path)
                .map_err(GradleBuildpackError::CannotSetGradleWrapperExecutableBit)?;

            let mut gradle_env = Env::from_current();
            handle_gradle_home_layer(&context, &mut gradle_env)?;

            print_section("Running Gradle build");

            track_subsection_timing(|| {
                print_subsection("Starting Gradle daemon");
                gradle_command::start_daemon(&gradle_wrapper_executable_path, &gradle_env)
                    .map_err(GradleBuildpackError::StartGradleDaemonError)
            })?;

            let project_tasks = track_subsection_timing(|| {
                print_subsection("Querying tasks");
                gradle_command::tasks(&context.app_dir, &gradle_env)
                    .map_err(|command_error| command_error.map_parse_error(|_| ()))
                    .map_err(GradleBuildpackError::GetTasksError)
            })?;

            let dependency_report = track_subsection_timing(|| {
                print_subsection("Querying dependency report");
                gradle_command::dependency_report(&context.app_dir, &gradle_env)
                    .map_err(GradleBuildpackError::GetDependencyReportError)
            })?;

            let task_name = buildpack_config
                .gradle_task
                .as_deref()
                .or_else(|| project_tasks.has_task("stage").then_some("stage"))
                .or_else(|| {
                    detect_framework(&dependency_report).map(|framework| match framework {
                        Framework::SpringBoot | Framework::Quarkus => "build",
                        Framework::Ratpack => "installDist",
                        Framework::Micronaut => "shadowJar",
                    })
                })
                .ok_or(GradleBuildpackError::BuildTaskUnknown)?;

            print_section("Running Gradle build");
            print_subsection(BuildpackOutputText::new(vec![
                BuildpackOutputTextSection::regular("Running "),
                BuildpackOutputTextSection::command(format!("./gradlew {task_name} -x check")),
            ]));

            let output = Command::new(&gradle_wrapper_executable_path)
                .current_dir(&context.app_dir)
                .envs(&gradle_env)
                .args([task_name, "-x", "check"])
                .output_and_write_streams(stdout(), stderr())
                .map_err(GradleBuildIoError)?;

            if !output.status.success() {
                Err(GradleBuildUnexpectedStatusError(output.status))?;
            }

            // Explicitly ignoring the result. If the daemon cannot be stopped, that is not a build
            // failure, nor can we recover from it in any way.
            let _ = gradle_command::stop_daemon(&gradle_wrapper_executable_path, &gradle_env);

            let process = default_app_process(&dependency_report, &context.app_dir)
                .map_err(GradleBuildpackError::CannotDetermineDefaultAppProcess)?;

            process
                .map_or(BuildResultBuilder::new(), |process| {
                    BuildResultBuilder::new().launch(LaunchBuilder::new().process(process).build())
                })
                .build()
        })
    }

    fn on_error(&self, error: libcnb::Error<Self::Error>) {
        libherokubuildpack::error::on_error(on_error_gradle_buildpack, error);
    }
}

buildpack_main!(GradleBuildpack);

impl From<GradleBuildpackError> for libcnb::Error<GradleBuildpackError> {
    fn from(e: GradleBuildpackError) -> Self {
        libcnb::Error::BuildpackError(e)
    }
}

const GRADLE_TASK_NAME_HEROKU_START_DAEMON: &str = "heroku_buildpack_start_daemon";
