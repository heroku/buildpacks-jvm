use crate::errors::on_error_maven_buildpack;
use crate::framework::DefaultAppProcessError;
use crate::layer::maven::handle_maven_layer;
use crate::layer::maven_repo::handle_maven_repository_layer;
use crate::mode::{determine_mode, Mode};
use crate::settings::{resolve_settings_xml_path, SettingsError};
use crate::warnings::{log_default_maven_version_warning, log_unused_maven_wrapper_warning};
use buildpacks_jvm_shared::system_properties::ReadSystemPropertiesError;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::launch::{LaunchBuilder, ProcessBuilder};
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::{buildpack_main, Buildpack, Env, Error, Platform};
use libherokubuildpack::download::DownloadError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use buildpacks_jvm_shared::output;
use buildpacks_jvm_shared::output::{BuildpackOutputText, BuildpackOutputTextSection};
#[cfg(test)]
use buildpacks_jvm_shared_test as _;
#[cfg(test)]
use java_properties as _;
#[cfg(test)]
use libcnb_test as _;

mod errors;
mod framework;
mod layer;
mod mode;
mod settings;
mod util;
mod warnings;

struct MavenBuildpack;

#[derive(Debug)]
enum MavenBuildpackError {
    UnsupportedMavenVersion(String),
    MavenTarballCreateTemporaryDirectoryError(std::io::Error),
    MavenTarballDownloadError(DownloadError),
    MavenTarballSha256IoError(std::io::Error),
    MavenTarballSha256Mismatch {
        expected_sha256: String,
        actual_sha256: String,
    },
    MavenTarballDecompressError(std::io::Error),
    CannotSplitMavenCustomOpts(shell_words::ParseError),
    CannotSplitMavenCustomGoals(shell_words::ParseError),
    DetermineModeError(ReadSystemPropertiesError),
    SettingsError(SettingsError),
    MavenFailedCommand(output::CmdError),
    CannotSetMavenWrapperExecutableBit(std::io::Error),
    DefaultAppProcessError(DefaultAppProcessError),
}

#[derive(Debug, Deserialize)]
struct MavenBuildpackMetadata {
    #[serde(rename = "default-version")]
    default_version: String,
    tarballs: HashMap<String, Tarball>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct Tarball {
    url: String,
    sha256: String,
}

impl Buildpack for MavenBuildpack {
    type Platform = GenericPlatform;
    type Metadata = MavenBuildpackMetadata;
    type Error = MavenBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        let app_has_pom = ["xml", "atom", "clj", "groovy", "rb", "scala", "yaml", "yml"]
            .iter()
            .map(|extension| context.app_dir.join(format!("pom.{extension}")))
            .any(|path| path.exists());

        if app_has_pom {
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

    #[allow(clippy::too_many_lines)]
    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let build_timer = Instant::now();
        output::print_buildpack_name("Heroku Maven Buildpack");

        let mut current_or_platform_env = Env::from_current();
        for (key, value) in context.platform.env() {
            current_or_platform_env.insert(key, value);
        }

        let mut mvn_env = Env::from_current();
        handle_maven_repository_layer(&context, &mut mvn_env)?;

        let maven_mode = determine_mode(
            &context.app_dir,
            &context.buildpack_descriptor.metadata.default_version,
        )
        .map_err(MavenBuildpackError::DetermineModeError)?;

        output::print_section("Installing Maven");

        let mvn_executable = match maven_mode {
            Mode::UseWrapper => {
                output::print_subsection("Skipping (Maven wrapper detected)");

                let maven_wrapper_path = context.app_dir.join("mvnw");

                fs::set_permissions(maven_wrapper_path, Permissions::from_mode(0o777))
                    .map_err(MavenBuildpackError::CannotSetMavenWrapperExecutableBit)?;

                PathBuf::from("./mvnw")
            }
            Mode::InstallVersion {
                version,
                warn_about_unused_maven_wrapper,
                warn_about_default_version,
            } => {
                if warn_about_unused_maven_wrapper {
                    log_unused_maven_wrapper_warning(&version);
                }

                if warn_about_default_version {
                    log_default_maven_version_warning(&version);
                }

                output::track_timing_subsection(
                    BuildpackOutputText::new(vec![
                        BuildpackOutputTextSection::regular("Selected Maven version "),
                        BuildpackOutputTextSection::value(&version),
                    ]),
                    || {
                        let tarball = context
                            .buildpack_descriptor
                            .metadata
                            .tarballs
                            .get(&version)
                            .cloned()
                            .ok_or_else(|| {
                                MavenBuildpackError::UnsupportedMavenVersion(version.clone())
                            })?;

                        handle_maven_layer(&context, &tarball, &mut mvn_env)
                    },
                )?;

                PathBuf::from("mvn")
            }
        };

        if let Some(java_home) = current_or_platform_env.get("JAVA_HOME") {
            mvn_env.insert("JAVA_HOME", java_home);
        }

        let maven_goals = current_or_platform_env
            .get("MAVEN_CUSTOM_GOALS")
            .map_or_else(
                || Ok(default_maven_goals()),
                |maven_custom_goals_string| {
                    shell_words::split(&maven_custom_goals_string.to_string_lossy())
                        .map_err(MavenBuildpackError::CannotSplitMavenCustomGoals)
                },
            )?;

        let mut maven_options = current_or_platform_env
            .get("MAVEN_CUSTOM_OPTS")
            .map_or_else(
                || Ok(default_maven_opts()),
                |maven_custom_opts_string| {
                    // Since this is a single environment variable, when users want to add multiple
                    // options, they will expect them to be split like a UNIX shell would. This means
                    // we need to support proper escaping for options that contain spaces.
                    shell_words::split(&maven_custom_opts_string.to_string_lossy())
                        .map_err(MavenBuildpackError::CannotSplitMavenCustomOpts)
                },
            )?;

        let settings_xml_path =
            resolve_settings_xml_path(&context.app_dir, &current_or_platform_env)
                .map_err(MavenBuildpackError::SettingsError)?;

        if let Some(settings_xml_path) = settings_xml_path {
            maven_options.push(String::from("-s"));
            maven_options.push(settings_xml_path.to_string_lossy().to_string());
        }

        // We need to set some options that relate to buildpack implementation internals. Those
        // options must not be overridden by the user via MAVEN_CUSTOM_OPTS for the buildpack to
        // work correctly. We also don't want to show them when we log the Maven command we're
        // running since they might be confusing to the user.
        let internal_maven_options = vec![String::from("-B")];

        output::print_section("Running Maven build");

        {
            let mut command = Command::new(&mvn_executable);
            command
                .current_dir(&context.app_dir)
                .args(
                    maven_options
                        .iter()
                        .chain(&internal_maven_options)
                        .chain(&maven_goals),
                )
                .envs(&mvn_env);

            output::run_command(command, false).map_err(MavenBuildpackError::MavenFailedCommand)?
        };
        output::track_timing_subsection(
            BuildpackOutputText::new(vec![
                BuildpackOutputTextSection::regular("Running "),
                BuildpackOutputTextSection::value(format!(
                    "{} dependency:list",
                    mvn_executable.to_string_lossy()
                )),
                BuildpackOutputTextSection::regular(" quietly"),
            ]),
            || {
                let mut command = Command::new(&mvn_executable);

                command
                    .current_dir(&context.app_dir)
                    .args(
                        maven_options.iter().chain(&internal_maven_options).chain(
                            [
                                format!(
                                    "-DoutputFile={}",
                                    app_dependency_list_path(&context.app_dir).to_string_lossy()
                                ),
                                String::from("dependency:list"),
                            ]
                            .iter(),
                        ),
                    )
                    .envs(&mvn_env);

                output::run_command(command, true).map_err(MavenBuildpackError::MavenFailedCommand)
            },
        )?;

        let mut build_result_builder = BuildResultBuilder::new();

        if let Some(process) = framework::default_app_process(&context.app_dir)
            .map_err(MavenBuildpackError::DefaultAppProcessError)?
        {
            build_result_builder =
                build_result_builder.launch(LaunchBuilder::new().process(process).build());
        }

        output::print_all_done(build_timer);
        build_result_builder.build()
    }

    fn on_error(&self, error: Error<Self::Error>) {
        libherokubuildpack::error::on_error(on_error_maven_buildpack, error);
    }
}

buildpack_main!(MavenBuildpack);

impl From<MavenBuildpackError> for libcnb::Error<MavenBuildpackError> {
    fn from(e: MavenBuildpackError) -> Self {
        libcnb::Error::BuildpackError(e)
    }
}

fn app_dependency_list_path<P: AsRef<Path>>(app_dir: P) -> PathBuf {
    app_dir.as_ref().join("target/mvn-dependency-list.log")
}

fn default_maven_goals() -> Vec<String> {
    vec![String::from("clean"), String::from("install")]
}

fn default_maven_opts() -> Vec<String> {
    vec![String::from("-DskipTests")]
}
