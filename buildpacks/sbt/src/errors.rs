use crate::configuration::ReadSbtBuildpackConfigurationError;
use crate::layers::sbt_extras::SbtExtrasLayerError;
use crate::layers::sbt_global::SbtGlobalLayerError;
use crate::sbt::output::SbtError;
use crate::sbt::version::ReadSbtVersionError;
use buildpacks_jvm_shared::log::{
    log_build_tool_unexpected_exit_code_error, log_please_try_again_error,
};
use buildpacks_jvm_shared::output::print_error;
use buildpacks_jvm_shared::system_properties::ReadSystemPropertiesError;
use indoc::formatdoc;
use semver::Version;
use std::fmt::Debug;
use std::process::ExitStatus;

#[derive(Debug)]
pub(crate) enum SbtBuildpackError {
    SbtExtrasLayerError(SbtExtrasLayerError),
    SbtGlobalLayerError(SbtGlobalLayerError),
    ReadSbtVersionError(ReadSbtVersionError),
    UnknownSbtVersion,
    UnsupportedSbtVersion(Version),
    DetectPhaseIoError(std::io::Error),
    SbtBuildIoError(std::io::Error),
    SbtBuildUnexpectedExitStatus(ExitStatus, Option<SbtError>),
    ReadSbtBuildpackConfigurationError(ReadSbtBuildpackConfigurationError),
    ReadSystemPropertiesError(ReadSystemPropertiesError),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn log_user_errors(error: SbtBuildpackError) {
    match error {
        SbtBuildpackError::SbtGlobalLayerError(SbtGlobalLayerError::CouldNotWritePlugin(error)) => {
                    log_please_try_again_error(
                            "Unexpected I/O error",
                            "An unexpected error occurred while attempting write the Heroku plugin for sbt.",
                            error,
                        );
        }

        SbtBuildpackError::SbtExtrasLayerError(error) => {
            match error {
                SbtExtrasLayerError::CouldNotWriteScript(error) | SbtExtrasLayerError::CouldNotSetPermissions(error) | SbtExtrasLayerError::CouldNotCreateLaunchersDir(error) => log_please_try_again_error(
                    "Unexpected I/O error",
                    "An unexpected I/O error occurred while setting up sbt-extras.",
                    error,
                ),
            }
        }

        SbtBuildpackError::ReadSbtVersionError(error) => match error {
            ReadSbtVersionError::CouldNotReadBuildProperties(error) => log_please_try_again_error(
                "Unexpected I/O error",
                "Could not read your application's system.properties file due to an unexpected I/O error.",
                error
            ),

            ReadSbtVersionError::CouldNotParseBuildProperties(error) => log_please_try_again_error(
                "Unexpected I/O error",
                "Could not read your application's project/build.properties file due to an unexpected I/O error.",
                error
            ),

            ReadSbtVersionError::MissingVersionProperty => print_error(
                "No sbt version defined",
                formatdoc! { "
                Your scala project must include project/build.properties and define a value for
                the `sbt.version` property.
            " },
            ),

            ReadSbtVersionError::CouldNotParseVersion(version, error) => {
                print_error(
                    "Unexpected version parse error",
                    formatdoc! { "
                Failed to read the `sbt.version` ({version}) declared in project/build.properties. Please
                ensure this value is a valid semantic version identifier (see https://semver.org/).

                Details: {error}
            " },
                );
            }
        },
        SbtBuildpackError::UnsupportedSbtVersion(version) => print_error(
            "Unsupported sbt version",
            formatdoc! { "
                You have defined an unsupported `sbt.version` ({version}) in the project/build.properties
                file. You must use a version of sbt between 0.11.0 and 1.x.
            " },
        ),

        SbtBuildpackError::UnknownSbtVersion => print_error(
            "Unknown sbt version",
            formatdoc! { "
                The buildpack could not determine the sbt version of this project.
                You must have a `sbt.version` key in the project/build.properties file.
            " },
        ),

        SbtBuildpackError::ReadSbtBuildpackConfigurationError(error) => match error {

            ReadSbtBuildpackConfigurationError::InvalidTaskList(error)
            | ReadSbtBuildpackConfigurationError::InvalidPreTaskList(error) => print_error(
                "Could not parse list",
                formatdoc! {"
                Could not parse a value into a list of words.
                Please check for quoting and escaping mistakes and try again.

                Details: {error}
            " },
            ),

            ReadSbtBuildpackConfigurationError::InvalidSbtClean(error)
            | ReadSbtBuildpackConfigurationError::InvalidAvailableAtLaunch(error) => print_error(
                "Could not parse boolean",
                formatdoc! {"
                Could not parse a value into a 'true' or 'false' value.
                Please check for mistakes and try again.

                Details: {error}
            " },
            ),
        },

        SbtBuildpackError::ReadSystemPropertiesError(error) => {
            match error {
                ReadSystemPropertiesError::IoError(error) => log_please_try_again_error(
                    "Failed to read system.properties",
                    "An unexpected error occurred while reading the system.properties file.",
                    error,
                ),

                ReadSystemPropertiesError::ParseError(error) => {
                    print_error(
                        "Invalid system.properties file",
                        formatdoc! {"
                            Your system.properties file could not be parsed.
                            Please ensure it is properly formatted and try again.

                            Details: {error}
                        "}
                    );
                }
            }
        }

        SbtBuildpackError::SbtBuildIoError(error) => log_please_try_again_error(
            "Running sbt failed",
            formatdoc! { "
                An unexpected IO error occurred while running sbt.
            "}, error,
        ),

        SbtBuildpackError::SbtBuildUnexpectedExitStatus(exit_status, None) => log_build_tool_unexpected_exit_code_error("sbt", exit_status),

        SbtBuildpackError::SbtBuildUnexpectedExitStatus(_, Some(SbtError::MissingTask(task_name))) => print_error(
            "Failed to run sbt!",
            formatdoc! {"
                It looks like your build.sbt does not have a valid '{task_name}' task. Please reference our Dev Center article for
                information on how to create one:

                https://devcenter.heroku.com/articles/scala-support#build-behavior
            "},
        ),

        SbtBuildpackError::DetectPhaseIoError(error) => log_please_try_again_error(
            "Unexpected I/O error",
            "An unexpected error occurred during the detect phase.",
            error,
        ),
    }
}

impl From<SbtBuildpackError> for libcnb::Error<SbtBuildpackError> {
    fn from(value: SbtBuildpackError) -> Self {
        libcnb::Error::BuildpackError(value)
    }
}
