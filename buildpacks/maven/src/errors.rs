use crate::{MavenBuildpackError, SettingsError};
use buildpacks_jvm_shared::log::log_please_try_again_error;
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
        } => log_error(
            "Maven download checksum error",
            formatdoc! {"
                Maven distribution download succeeded, but the downloaded file's SHA256
                checksum {actual_sha256} did not match the expected checksum {expected_sha256}.

                Please try again. If this error persists, please contact us:
                https://help.heroku.com/
            ", actual_sha256 = actual_sha256, expected_sha256 = expected_sha256 },
        ),
        MavenBuildpackError::MavenTarballSha256IoError(error) => log_error(
            "Maven download checksum error",
            formatdoc! {"
                Maven distribution download succeeded, but an error occurred while verifying the
                SHA256 checksum of the downloaded file.

                Please try again. If this error persists, please contact us:
                https://help.heroku.com/

                Details: {error}
            ", error = error },
        ),
        MavenBuildpackError::MavenBuildUnexpectedExitCode(exit_status) => {
            let exit_code_string = exit_status
                .code()
                .map_or_else(|| String::from("<unknown>"), |exit_code| exit_code.to_string());

            log_error(
                "Failed to build app with Maven",
                formatdoc! {"
                    We're sorry this build is failing! If you can't find the issue in application code,
                    please submit a ticket so we can help: https://help.heroku.com/

                    Maven exit code was: {exit_code}
                ", exit_code = exit_code_string },
            );
        },
        MavenBuildpackError::MavenBuildIoError(error) => log_error(
            "Failed to build app with Maven",
            formatdoc! {"
                We're sorry this build is failing! If you can't find the issue in application code,
                please submit a ticket so we can help: https://help.heroku.com/

                Details: {error}
            ", error = error },
        ),
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
