use crate::{MavenBuildpackError, SettingsError};
use indoc::formatdoc;
use libherokubuildpack::log_error;

pub fn on_error_maven_buildpack(error: MavenBuildpackError) -> i32 {
    match error {
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
            )
        }
        MavenBuildpackError::SettingsError(SettingsError::DownloadError(url, error)) => log_error(
            "Download of settings.xml failed",
            formatdoc! {"
                You have set MAVEN_SETTINGS_URL to \"{url}\". We tried to download the file at this
                URL, but the download failed. Please verify that the given URL is correct and try again.

                Details: {error}
            ", url = url, error = error },
        ),
        MavenBuildpackError::MavenTarballDownloadError(error) => log_error(
            "Maven download failed",
            formatdoc! {"
                Could not download Maven distribution.

                Please try again. If this error persists, please contact us:
                https://help.heroku.com/

                Details: {error}
            ", error = error },
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
        MavenBuildpackError::MavenTarballDecompressError(error) => log_error(
            "Maven download failed",
            formatdoc! {"
                Could not download Maven distribution.

                Please try again. If this error persists, please contact us:
                https://help.heroku.com/

                Details: {error}
            ", error = error },
        ),

        MavenBuildpackError::MavenBuildUnexpectedExitCode(exit_status) => {
            let exit_code_string = exit_status
                .code()
                .map(|exit_code| exit_code.to_string())
                .unwrap_or(String::from("<unknown>"));

            log_error(
                "Failed to build app with Maven",
                formatdoc! {"
                We're sorry this build is failing! If you can't find the issue in application code,
                please submit a ticket so we can help: https://help.heroku.com/

                Maven exit code was: {exit_code}
            ", exit_code = exit_code_string },
            )
        }
        MavenBuildpackError::MavenBuildIoError(_) => {}
        MavenBuildpackError::CannotSetExecutableBit(_, _) => {}
        MavenBuildpackError::MavenTarballNormalizationError(_) => {}
        MavenBuildpackError::CannotSplitMavenCustomOpts(_) => {}
        MavenBuildpackError::CannotSplitMavenCustomGoals(_) => {}
        MavenBuildpackError::DetermineModeError(_) => {}
    }

    1
}
