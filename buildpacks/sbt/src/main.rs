// Enable rustc and Clippy lints that are disabled by default.
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

mod build_configuration;
mod errors;
mod file_tree;
mod layers;

use crate::build_configuration::{create_build_config, BuildConfiguration};
use crate::errors::ScalaBuildpackError::{
    AlreadyDefinedAsObject, MissingStageTask, SbtBuildIoError, SbtBuildUnexpectedExitCode,
};
use crate::errors::{log_user_errors, ScalaBuildpackError};
use crate::file_tree::create_file_tree;
use crate::layers::coursier_cache::CoursierCacheLayer;
use crate::layers::ivy_cache::IvyCacheLayer;
use crate::layers::sbt::SbtLayer;
use indoc::formatdoc;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::{BuildPlan, BuildPlanBuilder};
use libcnb::data::layer_name;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::layer_env::Scope;
use libcnb::{buildpack_main, Buildpack, Env, Error, Platform};
use libherokubuildpack::command::CommandExt;
use libherokubuildpack::error::on_error as on_buildpack_error;
use libherokubuildpack::log::{log_header, log_info, log_warning};
use std::io::{stderr, stdout};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub(crate) struct ScalaBuildpack;

impl Buildpack for ScalaBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = ScalaBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        if !detect_sbt(&context.app_dir) {
            return DetectResultBuilder::fail().build();
        }

        DetectResultBuilder::pass()
            .build_plan(
                BuildPlanBuilder::new()
                    .requires("jdk")
                    .provides("jvm-application")
                    .requires("jvm-application")
                    .build(),
            )
            .build()
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let build_config = create_build_config(&context.app_dir, context.platform.env())?;

        let env = Env::from_current();
        let env = create_coursier_cache_layer(&context, &env, &build_config)?;
        let env = create_ivy_cache_layer(&context, &env, &build_config)?;
        let env = create_sbt_layer(&context, &env, &build_config)?;

        cleanup_any_existing_native_packager_directories(&context.app_dir);
        run_sbt_tasks(&context.app_dir, &build_config, &env)?;
        cleanup_compilation_artifacts(&context.app_dir);

        BuildResultBuilder::new().build()
    }

    fn on_error(&self, error: Error<Self::Error>) {
        on_buildpack_error(log_user_errors, error);
    }
}

buildpack_main!(ScalaBuildpack);

fn detect_sbt(app_dir: &Path) -> bool {
    !create_file_tree(app_dir.to_path_buf())
        .include("*.sbt")
        .include("project/*.scala")
        .include("project/build.properties")
        .include(".sbt/*.scala")
        .get_files()
        .unwrap_or(vec![])
        .is_empty()
}

fn create_coursier_cache_layer(
    context: &BuildContext<ScalaBuildpack>,
    env: &Env,
    build_config: &BuildConfiguration,
) -> Result<Env, Error<ScalaBuildpackError>> {
    let coursier_cache_layer = context.handle_layer(
        layer_name!("coursier_cache"),
        CoursierCacheLayer {
            available_at_launch: build_config.sbt_available_at_launch,
        },
    )?;
    Ok(coursier_cache_layer.env.apply(Scope::Build, env))
}

fn create_ivy_cache_layer(
    context: &BuildContext<ScalaBuildpack>,
    env: &Env,
    build_config: &BuildConfiguration,
) -> Result<Env, Error<ScalaBuildpackError>> {
    let ivy_cache_layer = context.handle_layer(
        layer_name!("ivy_cache"),
        IvyCacheLayer {
            available_at_launch: build_config.sbt_available_at_launch,
        },
    )?;
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
            available_at_launch: build_config.sbt_available_at_launch,
            env: env.clone(),
        },
    )?;
    Ok(sbt_layer.env.apply(Scope::Build, env))
}

// the native package plugin produces binaries in the target/universal/stage directory which is not included
// in the list of directories to clean up at the end of the build since a Procfile may reference this
// location to provide the entry point for an application. wiping the directory before the application build
// kicks off will ensure that no leftover artifacts are being carried around between builds.
fn cleanup_any_existing_native_packager_directories(app_dir: &Path) {
    let native_package_directory = app_dir.join("target").join("universal").join("stage");
    if native_package_directory.exists() {
        let delete_operation = create_file_tree(native_package_directory).delete();
        if let Err(error) = delete_operation {
            log_warning(
                "Removal of native package directory failed",
                formatdoc! {"
                    This error should not affect your built application but it may cause the container image
                    to be larger than expected.

                    Details: {error:?}
                "},
            );
        }
    }
}

fn cleanup_compilation_artifacts(app_dir: &Path) {
    log_info("Dropping compilation artifacts from the build");
    let delete_operation = create_file_tree(app_dir.join("target"))
        .include("scala-*")
        .include("streams")
        .include("resolution-cache")
        .exclude("resolution-cache/reports")
        .exclude("resolution-cache/*-compile.xml")
        .delete();

    if let Err(error) = delete_operation {
        log_warning(
            "Removal of compilation artifacts failed",
            formatdoc! {"
                This error should not affect your built application but it may cause the container image
                to be larger than expected.

                Details: {error:?}
            " },
        );
    }
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
mod detect_sbt_tests {
    use crate::detect_sbt;
    use std::fs::{create_dir, write};
    use tempfile::tempdir;

    #[test]
    fn detect_sbt_fails_when_no_sbt_files_in_application_directory() {
        let app_dir = tempdir().unwrap();
        assert!(!detect_sbt(app_dir.path()));
    }

    #[test]
    fn detect_sbt_passes_when_an_sbt_file_is_found_in_application_directory() {
        let app_dir = tempdir().unwrap();
        write(app_dir.path().join("build.sbt"), "").unwrap();
        assert!(detect_sbt(app_dir.path()));
    }

    #[test]
    fn detect_sbt_passes_when_a_scala_file_is_found_in_the_sbt_project_directory() {
        let app_dir = tempdir().unwrap();
        let sbt_project_path = app_dir.path().join("project");
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("some-file.scala"), "").unwrap();
        assert!(detect_sbt(app_dir.path()));
    }

    #[test]
    fn detect_sbt_passes_when_hidden_sbt_directory_is_found_in_application_directory() {
        let app_dir = tempdir().unwrap();
        let dot_sbt = app_dir.path().join(".sbt");
        create_dir(&dot_sbt).unwrap();
        write(dot_sbt.join("some-file.scala"), "").unwrap();
        assert!(detect_sbt(app_dir.path()));
    }

    #[test]
    fn detect_sbt_passes_when_build_properties_file_is_found_in_the_sbt_project_directory() {
        let app_dir = tempdir().unwrap();
        let sbt_project_path = app_dir.path().join("project");
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("build.properties"), "").unwrap();
        assert!(detect_sbt(app_dir.path()));
    }
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
            sbt_available_at_launch: None,
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
            sbt_available_at_launch: None,
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
            sbt_available_at_launch: None,
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
            sbt_available_at_launch: None,
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
            sbt_available_at_launch: None,
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
            sbt_available_at_launch: None,
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
            sbt_available_at_launch: None,
            sbt_version: Version::new(0, 0, 0),
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["clean", "projectName/compile", "projectName/stage"]
        );
    }
}
