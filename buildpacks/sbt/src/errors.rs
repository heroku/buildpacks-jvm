use crate::build_configuration::SbtBuildpackConfigurationError;
use crate::sbt_version::ReadSbtVersionError;
use buildpacks_jvm_shared::log_please_try_again_error;
use indoc::formatdoc;
use libherokubuildpack::log::log_error;
use semver::Version;
use std::fmt::Debug;
use std::process::ExitStatus;

#[derive(Debug)]
pub(crate) enum ScalaBuildpackError {
    ReadSbtVersionError(ReadSbtVersionError),
    UnsupportedSbtVersion(Version),
    DetectPhaseIoError(std::io::Error),
    CouldNotWriteSbtExtrasScript(std::io::Error),
    CouldNotSetExecutableBitForSbtExtrasScript(std::io::Error),
    CouldNotWriteSbtWrapperScript(std::io::Error),
    CouldNotSetExecutableBitForSbtWrapperScript(std::io::Error),
    SbtBuildIoError(std::io::Error),
    SbtBuildUnexpectedExitCode(ExitStatus),
    SbtInstallIoError(std::io::Error),
    SbtInstallUnexpectedExitCode(ExitStatus),
    CouldNotWriteSbtPlugin(std::io::Error),
    NoBuildpackPluginAvailable(String),
    MissingStageTask,
    AlreadyDefinedAsObject,
    SbtBuildpackConfigurationError(SbtBuildpackConfigurationError),
}

#[allow(clippy::too_many_lines)]
pub(crate) fn log_user_errors(error: ScalaBuildpackError) {
    match error {
        ScalaBuildpackError::ReadSbtVersionError(error) => match error {
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

            ReadSbtVersionError::MissingVersionProperty => log_error(
                "No sbt version defined",
                formatdoc! { "
                Your scala project must include project/build.properties and define a value for
                the `sbt.version` property.
            " },
            ),

            ReadSbtVersionError::CouldNotParseVersion(version, error) => {
                log_error(
                    "Unexpected version parse error",
                    formatdoc! { "
                Failed to read the `sbt.version` ({version}) declared in project/build.properties. Please
                ensure this value is a valid semantic version identifier (see https://semver.org/).

                Details: {error}
            " },
                );
            }
        },
        ScalaBuildpackError::UnsupportedSbtVersion(version) => log_error(
            "Unsupported sbt version",
            formatdoc! { "
                You have defined an unsupported `sbt.version` ({version}) in the project/build.properties
                file. You must use a version of sbt between 0.11.0 and 1.x.
            " },
        ),

        ScalaBuildpackError::SbtBuildpackConfigurationError(error) => match error {

            SbtBuildpackConfigurationError::InvalidTaskList(error)
            | SbtBuildpackConfigurationError::InvalidPreTaskList(error) => log_error(
                "Could not parse list",
                formatdoc! {"
                Could not parse a value into a list of words.
                Please check for quoting and escaping mistakes and try again.

                Details: {error}
            " },
            ),

            SbtBuildpackConfigurationError::InvalidSbtClean(error)
            | SbtBuildpackConfigurationError::InvalidAvailableAtLaunch(error) => log_error(
                "Could not parse boolean",
                formatdoc! {"
                Could not parse a value into a 'true' or 'false' value.
                Please check for mistakes and try again.

                Details: {error}
            " },
            ),
        },

        ScalaBuildpackError::SbtBuildIoError(error) => log_error(
            "Running sbt failed",
            formatdoc! { "
                We're sorry this build is failing! If you can't find the issue in application code,
                please submit a ticket so we can help: https://help.heroku.com/

                Details: {error}
            " },
        ),

        ScalaBuildpackError::SbtBuildUnexpectedExitCode(exit_status) => log_error(
            "Running sbt failed",
            formatdoc! { "
                We're sorry this build is failing! If you can't find the issue in application code,
                please submit a ticket so we can help: https://help.heroku.com/

                sbt exit code was: {exit_code}
            ", exit_code = exit_code_string(exit_status) },
        ),

        ScalaBuildpackError::NoBuildpackPluginAvailable(version) => log_error(
            "Failed to install Heroku plugin for sbt",
            formatdoc! { "
                No Heroku plugins supporting this version of sbt ({version}).
            " },
        ),

        ScalaBuildpackError::MissingStageTask => log_error(
            "Failed to run sbt!",
            formatdoc! {"
                It looks like your build.sbt does not have a valid 'stage' task. Please reference our Dev Center article for
                information on how to create one:

                https://devcenter.heroku.com/articles/scala-support#build-behavior
            "},
        ),

        ScalaBuildpackError::AlreadyDefinedAsObject => log_error(
            "Failed to run sbt!",
            formatdoc! {"
                We're sorry this build is failing. It looks like you may need to run a clean build to remove any
                stale SBT caches. You can do this by setting a configuration variable like this:

                $ heroku config:set SBT_CLEAN=true

                Then deploy you application with 'git push' again. If the build succeeds you can remove the variable by running this command:

                $ heroku config:unset SBT_CLEAN

                If this does not resolve the problem, please submit a ticket so we can help:
                https://help.heroku.com
            "},
        ),

        ScalaBuildpackError::CouldNotWriteSbtExtrasScript(error) => log_please_try_again_error(
            "Failed to write sbt-extras script",
            "An unexpected error occurred while writing the sbt-extras script.",
            error,
        ),

        ScalaBuildpackError::CouldNotSetExecutableBitForSbtExtrasScript(error) => {
            log_please_try_again_error(
                "Unexpected I/O error",
                "Failed to set executable permissions for the sbt-extras script.",
                error,
            );
        }

        ScalaBuildpackError::CouldNotWriteSbtWrapperScript(error) => log_please_try_again_error(
            "Failed to write sbt-extras script",
            "An unexpected error occurred while writing the sbt wrapper script.",
            error,
        ),

        ScalaBuildpackError::CouldNotSetExecutableBitForSbtWrapperScript(error) => {
            log_please_try_again_error(
                "Unexpected I/O error",
                "Failed to set executable permissions for the sbt wrapper script.",
                error,
            );
        }

        ScalaBuildpackError::SbtInstallIoError(error) => log_please_try_again_error(
            "Failed to install sbt",
            "An unexpected error occurred while attempting to install sbt.",
            error,
        ),

        ScalaBuildpackError::SbtInstallUnexpectedExitCode(exit_status) => {
            log_please_try_again_error(
                "Failed to install sbt",
                formatdoc! { "
              An unexpected exit code was reported while attempting to install sbt.

              sbt exit code was: {exit_code}
            ", exit_code = exit_code_string(exit_status) },
                error,
            );
        }

        ScalaBuildpackError::CouldNotWriteSbtPlugin(error) => log_please_try_again_error(
            "Failed to install Heroku plugin for sbt",
            "An unexpected error occurred while attempting to install the Heroku plugin for sbt.",
            error,
        ),

        ScalaBuildpackError::DetectPhaseIoError(error) => log_please_try_again_error(
            "Unexpected I/O error",
            "An unexpected error occurred during the detect phase.",
            error,
        ),
    }
}

fn exit_code_string(exit_status: ExitStatus) -> String {
    exit_status
        .code()
        .map_or(String::from("<unknown>"), |code| code.to_string())
}

impl From<ScalaBuildpackError> for libcnb::Error<ScalaBuildpackError> {
    fn from(value: ScalaBuildpackError) -> Self {
        libcnb::Error::BuildpackError(value)
    }
}
