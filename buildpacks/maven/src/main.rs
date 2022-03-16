// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

extern crate core;

use crate::dependencies::Framework;
use crate::error::on_error_maven_buildpack;
use crate::layer::maven::MavenLayer;
use crate::layer::maven_repo::MavenRepositoryLayer;
use crate::mode::{determine_mode, DetermineModeError, Mode};
use crate::settings::{resolve_settings_xml_path, SettingsError};
use crate::warnings::{log_default_maven_version_warning, log_unused_maven_wrapper_warning};
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::launch::{Launch, Process, ProcessBuilder};
use libcnb::data::layer_name;
use libcnb::data::process_type;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::layer_env::Scope;
use libcnb::{buildpack_main, Buildpack, Env, Error, Platform};
use libherokubuildpack::{log_header, log_info, DownloadError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

mod dependencies;
mod error;
mod layer;
mod mode;
mod settings;
mod util;
mod warnings;

pub struct MavenBuildpack;

#[derive(Debug)]
pub enum MavenBuildpackError {
    UnsupportedMavenVersion(String),
    MavenTarballDownloadError(DownloadError),
    MavenTarballSha256IoError(std::io::Error),
    MavenTarballSha256Mismatch {
        expected_sha256: String,
        actual_sha256: String,
    },
    MavenTarballDecompressError(std::io::Error),
    MavenTarballNormalizationError(std::io::Error),
    CannotSplitMavenCustomOpts(shell_words::ParseError),
    CannotSplitMavenCustomGoals(shell_words::ParseError),
    DetermineModeError(DetermineModeError),
    SettingsError(SettingsError),
    MavenBuildUnexpectedExitCode(ExitStatus),
    MavenBuildIoError(std::io::Error),
    CannotSetExecutableBit(PathBuf, std::io::Error),
}

#[derive(Debug, Deserialize)]
pub struct MavenBuildpackMetadata {
    #[serde(rename = "default-version")]
    default_version: String,
    tarballs: HashMap<String, Tarball>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tarball {
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

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let maven_repository_layer =
            context.handle_layer(layer_name!("repository"), MavenRepositoryLayer)?;

        let maven_mode = determine_mode(
            &context.app_dir,
            &context.buildpack_descriptor.metadata.default_version,
        )
        .map_err(MavenBuildpackError::DetermineModeError)?;

        log_header("Installing Maven");

        let (mvn_executable, mvn_env) = match maven_mode {
            Mode::UseWrapper => {
                log_info("Maven wrapper detected, skipping installation.");

                let maven_wrapper_path = context.app_dir.join("mvnw");

                fs::set_permissions(&maven_wrapper_path, Permissions::from_mode(0o777)).map_err(
                    |error| MavenBuildpackError::CannotSetExecutableBit(maven_wrapper_path, error),
                )?;

                (PathBuf::from("./mvnw"), Env::from_current())
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

                log_info(format!("Selected Maven version: {}", &version));

                let maven_layer = context
                    .buildpack_descriptor
                    .metadata
                    .tarballs
                    .get(&version)
                    .cloned()
                    .ok_or_else(|| {
                        MavenBuildpackError::UnsupportedMavenVersion(version.clone()).into()
                    })
                    .and_then(|tarball| {
                        context.handle_layer(layer_name!("maven"), MavenLayer { tarball })
                    })?;

                log_info(format!("Successfully installed Apache Maven {}", &version));

                (
                    PathBuf::from("mvn"),
                    maven_layer.env.apply(Scope::Build, &Env::from_current()),
                )
            }
        };

        let maven_goals = context
            .platform
            .env()
            .get("MAVEN_CUSTOM_GOALS")
            .map_or_else(
                || Ok(default_maven_goals()),
                |maven_custom_goals_string| {
                    shell_words::split(&maven_custom_goals_string.to_string_lossy())
                        .map_err(MavenBuildpackError::CannotSplitMavenCustomGoals)
                },
            )?;

        let mut maven_options = context
            .platform
            .env()
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
            resolve_settings_xml_path(&context).map_err(MavenBuildpackError::SettingsError)?;

        if let Some(settings_xml_path) = settings_xml_path {
            maven_options.push(String::from("-s"));
            maven_options.push(settings_xml_path.to_string_lossy().to_string());
        }

        // We need to set some options that relate to buildpack implementation internals. Those
        // options must not be overridden by the user via MAVEN_CUSTOM_OPTS for the buildpack to
        // work correctly. We also don't want to show them when we log the Maven command we're
        // running since they might be confusing to the user.
        let internal_maven_options = vec![
            String::from("-B"),
            format!("-Duser.home={}", &context.app_dir.to_string_lossy()),
            format!(
                "-Dmaven.repo.local={}",
                maven_repository_layer.path.to_string_lossy()
            ),
        ];

        log_header("Executing Maven");
        log_info(format!(
            "$ {} {} {}",
            mvn_executable.to_string_lossy(),
            shell_words::join(&maven_options),
            shell_words::join(&maven_goals)
        ));

        util::run_command(
            Command::new(&mvn_executable)
                .current_dir(&context.app_dir)
                .args(
                    maven_options
                        .iter()
                        .chain(&internal_maven_options)
                        .chain(&maven_goals),
                )
                .envs(&mvn_env),
            MavenBuildpackError::MavenBuildIoError,
            MavenBuildpackError::MavenBuildUnexpectedExitCode,
        )?;

        util::run_command(
            Command::new(&mvn_executable)
                .current_dir(&context.app_dir)
                .args(
                    maven_options.iter().chain(&internal_maven_options).chain(
                        [
                            format!(
                                "-DoutputFile={}",
                                context
                                    .app_dir
                                    .join("target/mvn-dependency-list.log")
                                    .to_string_lossy()
                                    .to_string()
                            ),
                            String::from("dependency:list"),
                        ]
                        .iter(),
                    ),
                )
                .envs(&mvn_env),
            MavenBuildpackError::MavenBuildIoError,
            MavenBuildpackError::MavenBuildUnexpectedExitCode,
        )?;

        // Generate launch.toml
        let launch = dependencies::detect_framework(&context.app_dir)
            .unwrap()
            .and_then(|framework| {
                util::list_directory_contents(context.app_dir.join("target"))
                    .unwrap()
                    .iter()
                    .find(|path| {
                        path.file_name()
                            .map(|file_name| file_name.to_string_lossy().to_string())
                            .filter(|file_name| {
                                file_name.ends_with(".jar")
                                    && !file_name.ends_with("-sources.jar")
                                    && !file_name.ends_with("-javadoc.jar")
                            })
                            .is_some()
                    })
                    .map(|main_jar_file_path| match framework {
                        Framework::SpringBoot => {
                            format!(
                                "java -Dserver.port=$PORT $JAVA_OPTS -jar {}",
                                main_jar_file_path.to_string_lossy()
                            )
                        }
                        Framework::WildflySwarm => {
                            format!(
                                "java -Dsswarm.http.port=$PORT $JAVA_OPTS -jar {}",
                                main_jar_file_path.to_string_lossy()
                            )
                        }
                    })
                    .map(|command| {
                        Launch::new().process(
                            ProcessBuilder::new(process_type!("web"), command)
                                .default(true)
                                .build(),
                        )
                    })
            });

        let mut build_result_builder = BuildResultBuilder::new();

        if let Some(launch) = launch {
            build_result_builder = build_result_builder.launch(launch);
        }

        build_result_builder.build()
    }

    fn on_error(&self, error: Error<Self::Error>) -> i32 {
        libherokubuildpack::on_error_heroku(on_error_maven_buildpack, error)
    }
}

buildpack_main!(MavenBuildpack);

impl From<MavenBuildpackError> for libcnb::Error<MavenBuildpackError> {
    fn from(e: MavenBuildpackError) -> Self {
        libcnb::Error::BuildpackError(e)
    }
}

fn default_maven_goals() -> Vec<String> {
    vec![String::from("clean"), String::from("install")]
}

fn default_maven_opts() -> Vec<String> {
    vec![String::from("-DskipTests")]
}
