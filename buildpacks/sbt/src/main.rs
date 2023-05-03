// Enable rustc and Clippy lints that are disabled by default.
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

mod cleanup;
mod configuration;
mod detect;
mod errors;
mod layers;
mod sbt_version;

use crate::cleanup::{cleanup_compilation_artifacts, cleanup_native_packager_directories};
use crate::configuration::{read_sbt_buildpack_configuration, SbtBuildpackConfiguration};
use crate::detect::is_sbt_project_directory;
use crate::errors::{log_user_errors, SbtBuildpackError};
use crate::layers::dependency_resolver_home::{DependencyResolver, DependencyResolverHomeLayer};
use crate::layers::sbt_boot::SbtBootLayer;
use crate::layers::sbt_extras::SbtExtrasLayer;
use crate::layers::sbt_global::SbtGlobalLayer;
use crate::sbt_version::{is_supported_sbt_version, read_sbt_version};
use buildpacks_jvm_shared::extend_build_env;
use buildpacks_jvm_shared::system_properties::read_system_properties;
use indoc::formatdoc;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::layer_name;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::{buildpack_main, Buildpack, Env, Error, Platform};
use libherokubuildpack::command::CommandExt;
use libherokubuildpack::error::on_error as on_buildpack_error;
use libherokubuildpack::log::{log_header, log_info, log_warning};
use std::io::{stderr, stdout};
use std::path::PathBuf;
use std::process::Command;

pub(crate) struct SbtBuildpack;

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

        let sbt_version = read_sbt_version(&context.app_dir)
            .map_err(SbtBuildpackError::ReadSbtVersionError)
            .and_then(|version| version.ok_or(SbtBuildpackError::UnknownSbtVersion))?;

        if !is_supported_sbt_version(&sbt_version) {
            Err(SbtBuildpackError::UnsupportedSbtVersion(
                sbt_version.clone(),
            ))?;
        }

        let mut env = Env::from_current();

        let sbt_available_at_launch = buildpack_configuration
            .sbt_available_at_launch
            .unwrap_or_default();

        extend_build_env(
            context.handle_layer(
                layer_name!("ivy-home"),
                DependencyResolverHomeLayer {
                    available_at_launch: sbt_available_at_launch,
                    dependency_resolver: DependencyResolver::Ivy,
                },
            )?,
            &mut env,
        );

        extend_build_env(
            context.handle_layer(
                layer_name!("coursier-home"),
                DependencyResolverHomeLayer {
                    available_at_launch: sbt_available_at_launch,
                    dependency_resolver: DependencyResolver::Coursier,
                },
            )?,
            &mut env,
        );

        extend_build_env(
            context.handle_layer(
                layer_name!("sbt-extras"),
                SbtExtrasLayer {
                    available_at_launch: sbt_available_at_launch,
                },
            )?,
            &mut env,
        );

        extend_build_env(
            context.handle_layer(
                layer_name!("sbt-boot"),
                SbtBootLayer {
                    available_at_launch: sbt_available_at_launch,
                    for_sbt_version: sbt_version.clone(),
                },
            )?,
            &mut env,
        );

        extend_build_env(
            context.handle_layer(
                layer_name!("sbt-global"),
                SbtGlobalLayer {
                    available_at_launch: sbt_available_at_launch,
                    for_sbt_version: sbt_version,
                },
            )?,
            &mut env,
        );

        if let Err(error) = cleanup_native_packager_directories(&context.app_dir) {
            log_warning(
                "Removal of native package directory failed",
                formatdoc! {"
                    This error should not affect your built application but it may cause the container image
                    to be larger than expected.

                    Details: {error:?}
                "},
            );
        }

        run_sbt_tasks(&context.app_dir, &buildpack_configuration, &env)?;

        log_info("Dropping compilation artifacts from the build");
        if let Err(error) = cleanup_compilation_artifacts(&context.app_dir) {
            log_warning(
                "Removal of compilation artifacts failed",
                formatdoc! {"
                This error should not affect your built application but it may cause the container image
                to be larger than expected.

                Details: {error:?}
            " },
            );
        }

        BuildResultBuilder::new().build()
    }

    fn on_error(&self, error: Error<Self::Error>) {
        on_buildpack_error(log_user_errors, error);
    }
}

buildpack_main!(SbtBuildpack);

fn run_sbt_tasks(
    app_dir: &PathBuf,
    buildpack_configuration: &SbtBuildpackConfiguration,
    env: &Env,
) -> Result<(), SbtBuildpackError> {
    log_header("Building Scala project");

    let tasks = get_sbt_build_tasks(buildpack_configuration);
    log_info(format!("Running: sbt {}", shell_words::join(&tasks)));

    let output = Command::new("sbt")
        .current_dir(app_dir)
        .args(tasks)
        .envs(env)
        .output_and_write_streams(stdout(), stderr())
        .map_err(SbtBuildpackError::SbtBuildIoError)?;

    output.status.success().then_some(()).ok_or(
        extract_error_from_sbt_output(&output.stdout)
            .unwrap_or(SbtBuildpackError::SbtBuildUnexpectedExitCode(output.status)),
    )
}

fn extract_error_from_sbt_output(stdout: &[u8]) -> Option<SbtBuildpackError> {
    let stdout = String::from_utf8_lossy(stdout);

    if stdout.contains("Not a valid key: stage") {
        Some(SbtBuildpackError::MissingStageTask)
    } else if stdout.contains("is already defined as object") {
        Some(SbtBuildpackError::AlreadyDefinedAsObject)
    } else {
        None
    }
}

fn get_sbt_build_tasks(build_config: &SbtBuildpackConfiguration) -> Vec<String> {
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
mod test {
    use crate::configuration::SbtBuildpackConfiguration;
    use crate::errors::SbtBuildpackError;
    use crate::extract_error_from_sbt_output;
    use crate::get_sbt_build_tasks;
    use indoc::formatdoc;

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

        match extract_error_from_sbt_output(&stdout) {
            Some(SbtBuildpackError::MissingStageTask) => {}
            _ => panic!("expected ScalaBuildpackError::MissingStageTask"),
        };
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

        match extract_error_from_sbt_output(&stdout) {
            Some(SbtBuildpackError::AlreadyDefinedAsObject) => {}
            _ => panic!("expected ScalaBuildpackError::AlreadyDefinedAsObject"),
        };
    }

    #[test]
    fn get_sbt_build_tasks_with_no_configured_options() {
        let config = SbtBuildpackConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: None,
            sbt_available_at_launch: None,
        };
        assert_eq!(get_sbt_build_tasks(&config), vec!["compile", "stage"]);
    }

    #[test]
    fn get_sbt_build_tasks_with_all_configured_options() {
        let config = SbtBuildpackConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: Some(vec!["preTask".to_string()]),
            sbt_tasks: Some(vec!["task".to_string()]),
            sbt_clean: Some(true),
            sbt_available_at_launch: None,
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["clean", "preTask", "task"]
        );
    }

    #[test]
    fn get_sbt_build_tasks_with_clean_set_to_true() {
        let config = SbtBuildpackConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(true),
            sbt_available_at_launch: None,
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["clean", "compile", "stage"]
        );
    }

    #[test]
    fn get_sbt_build_tasks_with_clean_set_to_false() {
        let config = SbtBuildpackConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(false),
            sbt_available_at_launch: None,
        };
        assert_eq!(get_sbt_build_tasks(&config), vec!["compile", "stage"]);
    }

    #[test]
    fn get_sbt_build_tasks_with_project_set() {
        let config = SbtBuildpackConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: None,
            sbt_available_at_launch: None,
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["projectName/compile", "projectName/stage"]
        );
    }

    #[test]
    fn get_sbt_build_tasks_with_project_and_pre_tasks_set() {
        let config = SbtBuildpackConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: Some(vec!["preTask".to_string()]),
            sbt_tasks: None,
            sbt_clean: None,
            sbt_available_at_launch: None,
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["preTask", "projectName/compile", "projectName/stage"]
        );
    }

    #[test]
    fn get_sbt_build_tasks_with_project_and_clean_set() {
        let config = SbtBuildpackConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(true),
            sbt_available_at_launch: None,
        };
        assert_eq!(
            get_sbt_build_tasks(&config),
            vec!["clean", "projectName/compile", "projectName/stage"]
        );
    }
}
