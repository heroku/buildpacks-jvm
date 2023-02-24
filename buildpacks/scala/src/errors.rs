use indoc::formatdoc;
use libcnb::Error;
use libherokubuildpack::log::log_error;
use std::ffi::OsString;
use std::fmt::Debug;
use std::process::ExitStatus;

#[derive(Debug)]
pub enum ScalaBuildpackError {
    CouldNotWriteSbtExtrasScript(std::io::Error),
    CouldNotSetExecutableBitForSbtExtrasScript(std::io::Error),
    CouldNotWriteSbtWrapperScript(std::io::Error),
    CouldNotSetExecutableBitForSbtWrapperScript(std::io::Error),
    MissingSbtBuildPropertiesFile,
    SbtPropertiesFileReadError(std::io::Error),
    InvalidSbtPropertiesFile(java_properties::PropertiesError),
    MissingDeclaredSbtVersion,
    UnsupportedSbtVersion(String),
    SbtVersionNotInSemverFormat(String, semver::Error),
    SbtBuildIoError(std::io::Error),
    SbtBuildUnexpectedExitCode(ExitStatus),
    SbtInstallIoError(std::io::Error),
    SbtInstallUnexpectedExitCode(ExitStatus),
    CouldNotWriteSbtPlugin(std::io::Error),
    NoBuildpackPluginAvailable(String),
    CouldNotParseBooleanFromProperty(String, std::str::ParseBoolError),
    CouldNotParseBooleanFromEnvironment(String, std::str::ParseBoolError),
    CouldNotParseListConfigurationFromProperty(String, shell_words::ParseError),
    CouldNotParseListConfigurationFromEnvironment(String, shell_words::ParseError),
    CouldNotConvertEnvironmentValueIntoString(String, OsString),
    CouldNotReadSbtOptsFile(std::io::Error),
    CouldNotParseListConfigurationFromSbtOptsFile(shell_words::ParseError),
    MissingStageTask,
    AlreadyDefinedAsObject,
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

        ScalaBuildpackError::NoBuildpackPluginAvailable(version) => log_error(
            "Failed to install Heroku plugin for sbt",
            formatdoc! { "
                No Heroku plugins supporting this version of sbt ({version}).
            ", version = version },
        ),

        ScalaBuildpackError::CouldNotParseListConfigurationFromProperty(property_name, error) => log_error(
            format!("Invalid {property_name} property"),
            formatdoc! {"
                Could not parse the value of the `{property_name}` property from the system.properties file into a list of words.
                Please check the `{property_name}` property for quoting and escaping mistakes and try again.

                Details: {error}
            ", property_name = property_name, error = error }
        ),

        ScalaBuildpackError::CouldNotParseListConfigurationFromEnvironment(variable_name, error) => log_error(
            format!("Invalid {variable_name} environment variable"),
            formatdoc! {"
                Could not parse the value of the {variable_name} environment variable into a list of words.
                Please check {variable_name} for quoting and escaping mistakes and try again.

                Details: {error}
            ", variable_name = variable_name, error = error }
        ),

        ScalaBuildpackError::CouldNotParseBooleanFromProperty(property_name, error) => log_error(
            format! ("Invalid {property_name} property"),
            formatdoc! {"
                Could not parse the value of the `{property_name}` property from the system.properties file into a 'true' or 'false' value.
                Please check `{property_name}` for mistakes and try again.

                Details: {error}
            ", property_name = property_name, error = error }
        ),

        ScalaBuildpackError::CouldNotParseBooleanFromEnvironment(variable_name, error) => log_error(
            format!("Invalid {variable_name} environment variable"),
            formatdoc! {"
                Could not parse the value of {variable_name} environment variable into a 'true' or 'false' value.
                Please check {variable_name} for mistakes and try again.

                Details: {error}
            ", variable_name = variable_name, error = error }
        ),

        ScalaBuildpackError::CouldNotConvertEnvironmentValueIntoString(variable_name, value) => log_error(
            format!("Invalid {variable_name} environment variable"),
            formatdoc! {"
                Could not convert the value of the environment variable {variable_name} into a string. Please
                check that the value of {variable_name} only contains Unicode characters and try again.

                Value: {value}
            ", variable_name = variable_name, value = value.to_string_lossy() }
        ),

        ScalaBuildpackError::CouldNotParseListConfigurationFromSbtOptsFile(error) => log_error(
            "Invalid .sbtopts file",
            formatdoc! {"
                Could not read the value of the .sbtopts file into a list of arguments. Please check
                the file for mistakes and please try again.

                Details: {error}
            ", error = error }
        ),

        ScalaBuildpackError::MissingStageTask => log_error(
            "Failed to run sbt!",
            formatdoc! {"
                It looks like your build.sbt does not have a valid 'stage' task. Please read our Dev Center article for
                information on how to create one:
                https://devcenter.heroku.com/articles/scala-support#build-behavior
            "}
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
            "}
        ),

        ScalaBuildpackError::CouldNotWriteSbtExtrasScript(error) => log_please_try_again_error(
            "Failed to write sbt-extras script",
            "An unexpected error occurred while writing the sbt-extras script.",
            error,
        ),

        ScalaBuildpackError::CouldNotSetExecutableBitForSbtExtrasScript(error) => log_please_try_again_error(
            "Unexpected I/O error",
            "Failed to set executable permissions for the sbt-extras script.",
            error
        ),

        ScalaBuildpackError::CouldNotWriteSbtWrapperScript(error) => log_please_try_again_error(
            "Failed to write sbt-extras script",
            "An unexpected error occurred while writing the sbt wrapper script.",
            error,
        ),

        ScalaBuildpackError::CouldNotSetExecutableBitForSbtWrapperScript(error) => log_please_try_again_error(
            "Unexpected I/O error",
            "Failed to set executable permissions for the sbt wrapper script.",
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

        ScalaBuildpackError::CouldNotReadSbtOptsFile(error) => log_please_try_again_error(
            "Unexpected I/O error",
            "Could not read your application's .sbtopts file due to an unexpected I/O error.",
            error
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
