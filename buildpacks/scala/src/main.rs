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
use crate::errors::ScalaBuildpackError::{
    AlreadyDefinedAsObject, MissingStageTask, SbtBuildIoError, SbtBuildUnexpectedExitCode,
};
use crate::errors::{log_user_errors, ScalaBuildpackError};
use crate::layers::coursier_cache::CoursierCacheLayer;
use crate::layers::ivy_cache::IvyCacheLayer;
use crate::layers::sbt::SbtLayer;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::{BuildPlan, BuildPlanBuilder};
use libcnb::data::layer_name;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::layer_env::Scope;
use libcnb::{buildpack_main, Buildpack, Env, Error, Platform};
use libherokubuildpack::command::CommandExt;
use libherokubuildpack::error::on_error as on_buildpack_error;
use libherokubuildpack::log::{log_header, log_info};
use std::io::{stderr, stdout};
use std::path::PathBuf;
use std::process::{Command, Output};

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
        let build_config = create_build_config(&context.app_dir, context.platform.env())?;

        let env = Env::from_current();
        let env = create_coursier_cache_layer(&context, &env)?;
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
        .provides("jvm-application")
        .requires("jvm-application")
        .build()
}

fn create_coursier_cache_layer(
    context: &BuildContext<ScalaBuildpack>,
    env: &Env,
) -> Result<Env, Error<ScalaBuildpackError>> {
    let coursier_cache_layer =
        context.handle_layer(layer_name!("coursier_cache"), CoursierCacheLayer)?;
    Ok(coursier_cache_layer.env.apply(Scope::Build, env))
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
    let sbt_layer = context.handle_layer(
        layer_name!("sbt"),
        SbtLayer {
            sbt_version: build_config.sbt_version.clone(),
            sbt_opts: build_config.sbt_opts.clone(),
            env: env.clone(),
        },
    )?;
    Ok(sbt_layer.env.apply(Scope::Build, env))
}

fn run_sbt_tasks(
    app_dir: &PathBuf,
    build_config: &BuildConfiguration,
    env: &Env,
) -> Result<(), ScalaBuildpackError> {
    log_header("Building Scala project");

    let tasks = get_sbt_build_tasks(build_config);
    log_info(format!("Running: sbt {}", shell_words::join(&tasks)));

    let output = Command::new("sbt")
        .current_dir(app_dir)
        .args(tasks)
        .envs(env)
        .output_and_write_streams(stdout(), stderr())
        .map_err(SbtBuildIoError)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(handle_sbt_error(&output))
    }
}

fn handle_sbt_error(output: &Output) -> ScalaBuildpackError {
    if let Ok(stdout) = std::str::from_utf8(&output.stdout) {
        if stdout.contains("Not a valid key: stage") {
            return MissingStageTask;
        }
        if stdout.contains("is already defined as object") {
            return AlreadyDefinedAsObject;
        }
    }
    SbtBuildUnexpectedExitCode(output.status)
}

#[cfg(test)]
mod handle_sbt_error_tests {
    use crate::errors::ScalaBuildpackError;
    use crate::errors::ScalaBuildpackError::MissingStageTask;
    use crate::handle_sbt_error;
    use indoc::formatdoc;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus, Output};

    #[test]
    fn check_missing_stage_error_is_reported() {
        let stdout = formatdoc! {"
            [error] Expected ';'
            [error] Not a valid command: stage (similar: last-grep, set, last)
            [error] Not a valid project ID: stage
            [error] Expected ':'
            [error] Not a valid key: stage (similar: state, target, tags)
            [error] stage
            [error]      ^
        "}
        .into_bytes();

        let output = Output {
            stdout,
            stderr: vec![],
            status: ExitStatus::from_raw(0),
        };
        let err = handle_sbt_error(&output);
        match err {
            MissingStageTask => {}
            _ => panic!("expected MissingStageTask error"),
        }
    }

    #[test]
    fn check_already_defined_as_error_is_reported() {
        let stdout = formatdoc! {"
            [error] Expected ';'
            [error] Not a valid command: stage (similar: last-grep, set, last)
            [error] Not a valid project ID: stage
            [error] Expected ':'
            [error] Blah is already defined as object Blah
        "}
        .into_bytes();

        let output = Output {
            stdout,
            stderr: vec![],
            status: ExitStatus::from_raw(0),
        };
        let err = handle_sbt_error(&output);
        match err {
            ScalaBuildpackError::AlreadyDefinedAsObject => {}
            _ => panic!("expected MissingStageTask error"),
        }
    }
}

fn get_sbt_build_tasks(build_config: &BuildConfiguration) -> Vec<String> {
    let mut tasks: Vec<String> = Vec::new();

    if let Some(true) = &build_config.sbt_clean {
        tasks.push(String::from("clean"));
    }

    if let Some(sbt_pre_tasks) = &build_config.sbt_pre_tasks {
        sbt_pre_tasks
            .iter()
            .for_each(|task| tasks.push(task.to_string()));
    }

    if let Some(sbt_tasks) = &build_config.sbt_tasks {
        sbt_tasks
            .iter()
            .for_each(|task| tasks.push(task.to_string()));
    } else {
        let default_tasks = vec![String::from("compile"), String::from("stage")];
        for default_task in &default_tasks {
            tasks.push(match &build_config.sbt_project {
                Some(project) => format!("{project}/{default_task}"),
                None => default_task.to_string(),
            });
        }
    }

    tasks
}

#[cfg(test)]
mod get_sbt_build_tasks_tests {
    use crate::build_configuration::BuildConfiguration;
    use crate::get_sbt_build_tasks;
    use semver::Version;

    #[test]
    fn get_sbt_build_tasks_with_no_configured_options() {
        let config = BuildConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: None,
            sbt_opts: None,
            sbt_version: Version::new(0, 0, 0),
        };
        assert_eq!(get_sbt_build_tasks(&config), vec!["compile", "stage"]);
    }

    #[test]
    fn get_sbt_build_tasks_with_all_configured_options() {
        let config = BuildConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: Some(vec!["preTask".to_string()]),
            sbt_tasks: Some(vec!["task".to_string()]),
            sbt_clean: Some(true),
            sbt_opts: None,
            sbt_version: Version::new(0, 0, 0),
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["clean", "preTask", "task"]
        );
    }

    #[test]
    fn get_sbt_build_tasks_with_clean_set_to_true() {
        let config = BuildConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(true),
            sbt_opts: None,
            sbt_version: Version::new(0, 0, 0),
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["clean", "compile", "stage"]
        );
    }

    #[test]
    fn get_sbt_build_tasks_with_clean_set_to_false() {
        let config = BuildConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(false),
            sbt_opts: None,
            sbt_version: Version::new(0, 0, 0),
        };
        assert_eq!(get_sbt_build_tasks(&config), vec!["compile", "stage"]);
    }

    #[test]
    fn get_sbt_build_tasks_with_project_set() {
        let config = BuildConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: None,
            sbt_opts: None,
            sbt_version: Version::new(0, 0, 0),
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["projectName/compile", "projectName/stage"]
        );
    }

    #[test]
    fn get_sbt_build_tasks_with_project_and_pre_tasks_set() {
        let config = BuildConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: Some(vec!["preTask".to_string()]),
            sbt_tasks: None,
            sbt_clean: None,
            sbt_opts: None,
            sbt_version: Version::new(0, 0, 0),
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["preTask", "projectName/compile", "projectName/stage"]
        );
    }

    #[test]
    fn get_sbt_build_tasks_with_project_and_clean_set() {
        let config = BuildConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(true),
            sbt_opts: None,
            sbt_version: Version::new(0, 0, 0),
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["clean", "projectName/compile", "projectName/stage"]
        );
    }
}
