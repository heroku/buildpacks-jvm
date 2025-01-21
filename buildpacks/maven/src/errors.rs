use crate::{MavenBuildpackError, SettingsError};
use buildpacks_jvm_shared::log::{
    log_build_tool_io_error, log_build_tool_unexpected_exit_code_error, log_please_try_again,
    log_please_try_again_error,
};
use buildpacks_jvm_shared::output::CmdError;
use buildpacks_jvm_shared::system_properties::ReadSystemPropertiesError;
use indoc::formatdoc;
use libherokubuildpack::log::log_error;

#[allow(clippy::too_many_lines)]
pub(crate) fn on_error_maven_buildpack(error: MavenBuildpackError) {
    match error {
        MavenBuildpackError::DetermineModeError(ReadSystemPropertiesError::IoError(error)) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not read your application's system.properties file due to an unexpected I/O error.",
            error,
        ),
        MavenBuildpackError::MavenTarballCreateTemporaryDirectoryError(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not create a temporary directory for Maven distribution",
            error
        ),
        MavenBuildpackError::MavenTarballDownloadError(error) => log_please_try_again_error(
            "Maven download failed",
            "Could not download Maven distribution.",
            error,
        ),
        MavenBuildpackError::MavenTarballDecompressError(error) => log_please_try_again_error(
            "Maven download failed",
            "Could not download Maven distribution.",
            error,
        ),
        MavenBuildpackError::CannotSetMavenWrapperExecutableBit(error) => log_please_try_again_error(
            "Failed to set executable bit for Maven wrapper",
            "Failed to set executable bit for Maven wrapper",
            error,
        ),
        MavenBuildpackError::DefaultAppProcessError(error) => log_please_try_again_error(
            "Could not determine default process",
            "While trying to determine a default process based on the used application framework, an unexpected error occurred.",
            error,
        ),
        MavenBuildpackError::UnsupportedMavenVersion(version) => log_error(
            "Unsupported Maven version",
            formatdoc! {"
                You have defined an unsupported Maven version ({version}) in the system.properties file.
            ", version = version },
        ),
        MavenBuildpackError::SettingsError(SettingsError::InvalidMavenSettingsPath(path)) => {
            log_error(
                "Cannot find custom settings.xml file",
                formatdoc! {"
                    You have set MAVEN_SETTINGS_PATH to \"{path}\". We could not find that file in your app.
                    Please verify the path is correct, ensure you committed this file to your app and then try again.
                ", path = path.to_string_lossy() },
            );
        },
        MavenBuildpackError::SettingsError(SettingsError::DownloadError(url, error)) => log_error(
            "Download of settings.xml failed",
            formatdoc! {"
                You have set MAVEN_SETTINGS_URL to \"{url}\". We tried to download the file at this
                URL, but the download failed. Please verify that the given URL is correct and try again.

                Details: {error}
            ", url = url, error = error },
        ),
        MavenBuildpackError::MavenTarballSha256Mismatch {
            expected_sha256,
            actual_sha256,
        } => log_please_try_again(
            "Maven download checksum error",
            formatdoc! {"
                Maven distribution download succeeded, but the downloaded file's SHA256
                checksum {actual_sha256} did not match the expected checksum {expected_sha256}.
            ", actual_sha256 = actual_sha256, expected_sha256 = expected_sha256 },
        ),
        MavenBuildpackError::MavenTarballSha256IoError(error) => log_please_try_again_error(
            "Maven download checksum error",
            formatdoc! {"
                Maven distribution download succeeded, but an error occurred while verifying the
                SHA256 checksum of the downloaded file.
            "},
            error
        ),
        MavenBuildpackError::MavenFailedCommand(error) => {
            match error {
                CmdError::SystemError(_, error) => log_build_tool_io_error("Maven", error),
                CmdError::NonZeroExitNotStreamed(named_output) |
                CmdError::NonZeroExitAlreadyStreamed(named_output) => log_build_tool_unexpected_exit_code_error("Maven",*named_output.status()),
            }
        }
        MavenBuildpackError::CannotSplitMavenCustomOpts(error) => log_error(
            "Invalid MAVEN_CUSTOM_OPTS",
            formatdoc! {"
                Could not split the value of the MAVEN_CUSTOM_OPTS environment variable into separate
                Maven options. Please check MAVEN_CUSTOM_OPTS for quoting and escaping mistakes and try again.

                Details: {error}
            ", error = error },
        ),
        MavenBuildpackError::CannotSplitMavenCustomGoals(error) => log_error(
            "Invalid MAVEN_CUSTOM_GOALS",
            formatdoc! {"
                Could not split the value of the MAVEN_CUSTOM_GOALS environment variable into separate
                Maven goals. Please check MAVEN_CUSTOM_GOALS for quoting and escaping mistakes and try again.

                Details: {error}
            ", error = error },
        ),
        MavenBuildpackError::DetermineModeError(
            ReadSystemPropertiesError::ParseError(error),
        ) => log_error(
            "Invalid system.properties file",
            formatdoc! {"
                Could not parse your application's system.properties file. Please ensure that your
                system.properties file is a valid Java properties file and try again.

                Details: {error}
            ", error = error },
        ),
    }
}
