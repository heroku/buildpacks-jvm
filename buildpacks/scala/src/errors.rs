use indoc::formatdoc;
use libcnb::Error;
use libherokubuildpack::log::log_error;
use std::fmt::Debug;
use std::process::ExitStatus;

#[derive(thiserror::Error, Debug)]
pub enum ScalaBuildpackError {
    #[error("Could not write runtime script to layer: {0}")]
    CouldNotWriteSbtExtrasScript(std::io::Error),

    #[error("Could not set executable bit on runtime script: {0}")]
    CouldNotSetExecutableBitForSbtExtrasScript(std::io::Error),

    #[error("TODO")]
    MissingSbtBuildPropertiesFile,

    #[error("TODO")]
    SbtPropertiesFileReadError(std::io::Error),

    #[error("TODO")]
    InvalidSbtPropertiesFile(java_properties::PropertiesError),

    #[error("TODO")]
    MissingDeclaredSbtVersion,

    #[error("TODO")]
    UnsupportedSbtVersion(String),

    #[error("TODO")]
    SbtVersionNotInSemverFormat(String, semver::Error),

    #[error("TODO")]
    SbtBuildIoError(std::io::Error),

    #[error("TODO")]
    SbtBuildUnexpectedExitCode(ExitStatus),

    #[error("TODO")]
    SbtInstallIoError(std::io::Error),

    #[error("TODO")]
    SbtInstallUnexpectedExitCode(ExitStatus),

    #[error("TODO")]
    CouldNotWriteSbtPlugin(std::io::Error),

    #[error("TODO")]
    NoBuildpackPluginAvailable(String),
}

#[allow(clippy::too_many_lines)]
pub fn log_user_errors(error: ScalaBuildpackError) {
    match error {
        ScalaBuildpackError::MissingDeclaredSbtVersion |
        ScalaBuildpackError::MissingSbtBuildPropertiesFile => log_error(
            "No sbt version defined",
            formatdoc! { "
                Your scala project must include project/build.properties and define a value for
                the `sbt.version` property.
            " },
        ),

        ScalaBuildpackError::UnsupportedSbtVersion(version) => log_error(
            "Unsupported sbt version",
            formatdoc! { "
                You have defined an unsupported `sbt.version` ({version}) in the project/build.properties
                file. You must use a version of sbt between 0.11.0 and 1.x.
            ", version = version },
        ),

        ScalaBuildpackError::SbtBuildIoError(error) => log_error(
            "Failed to build app with sbt",
            formatdoc! { "
                We're sorry this build is failing!  If you can't find the issue in application code,
                please submit a ticket so we can help: https://help.heroku.com/

                Details: {error}
            ", error = error }
        ),

        ScalaBuildpackError::SbtBuildUnexpectedExitCode(exit_status) => log_error(
            "Failed to build app with sbt",
            formatdoc! { "
                We're sorry this build is failing! If you can't find the issue in application code,
                please submit a ticket so we can help: https://help.heroku.com/

                sbt exit code was: {exit_code}
            ", exit_code = get_exit_code(exit_status) }
        ),

        ScalaBuildpackError::SbtVersionNotInSemverFormat(version, error) => log_error(
            "Unexpected version parse error",
            formatdoc! { "
                Failed to read the `sbt.version` ({version}) declared in project/build.properties. Please
                ensure this value is a valid semantic version identifier (see https://semver.org/).

                Details: {error}
            ", version = version, error = error },
        ),

        ScalaBuildpackError::CouldNotWriteSbtExtrasScript(error) => log_please_try_again_error(
            "Failed to write sbt-extras script",
            "An unexpected error occurred while writing the sbt-extras script.",
            error,
        ),

        ScalaBuildpackError::CouldNotSetExecutableBitForSbtExtrasScript(error) => log_please_try_again_error(
            "Unexpected I/O error",
            "Failed to set executable permissions for sbt-extras script.",
            error
        ),

        ScalaBuildpackError::SbtPropertiesFileReadError(error) => log_please_try_again_error(
            "Unexpected I/O error",
            "Could not read your application's system.properties file due to an unexpected I/O error.",
            error
        ),

        ScalaBuildpackError::InvalidSbtPropertiesFile(error) => log_please_try_again_error(
            "Unexpected I/O error",
            "Could not read your application's project/build.properties file due to an unexpected I/O error.",
            error
        ),

        ScalaBuildpackError::SbtInstallIoError(error) => log_please_try_again_error(
            "Failed to install sbt",
            "An unexpected error occurred while attempting to install sbt.",
            error
        ),

        ScalaBuildpackError::SbtInstallUnexpectedExitCode(exit_status) => log_please_try_again_error(
            "Failed to install sbt",
            formatdoc! { "
              An unexpected exit code was reported while attempting to install sbt.

              sbt exit code was: {exit_code}
            ", exit_code = get_exit_code(exit_status) },
            error
        ),

        ScalaBuildpackError::CouldNotWriteSbtPlugin(error) => log_please_try_again_error(
            "Failed to install Heroku plugin for sbt",
            "An unexpected error occurred while attempting to install the Heroku plugin for sbt.",
            error
        ),

        ScalaBuildpackError::NoBuildpackPluginAvailable(version) => log_error(
            "Failed to install Heroku plugin for sbt",
            formatdoc! { "
                No Heroku plugins supporting this version of sbt ({version}).
            ", version = version },
        ),
    }
}

fn log_please_try_again_error<H: AsRef<str>, M: AsRef<str>, E: Debug>(
    header: H,
    message: M,
    error: E,
) {
    log_error(
        header,
        formatdoc! {"
            {message}

            Please try again. If this error persists, please contact us:
            https://help.heroku.com/

            Details: {error:?}
        ", message = message.as_ref(), error = error },
    );
}

fn get_exit_code(exit_status: ExitStatus) -> String {
    exit_status
        .code()
        .map_or_else(|| String::from("<unknown>"), |code| code.to_string())
}

impl From<ScalaBuildpackError> for Error<ScalaBuildpackError> {
    fn from(value: ScalaBuildpackError) -> Self {
        Error::BuildpackError(value)
    }
}
