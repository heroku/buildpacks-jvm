use crate::{NormalizeVersionStringError, OpenJdkBuildpackError, ValidateSha256Error};
use buildpacks_jvm_shared::log_please_try_again_error;
use buildpacks_jvm_shared::system_properties::ReadSystemPropertiesError;
use indoc::formatdoc;
use libherokubuildpack::download::DownloadError;
use libherokubuildpack::log::log_error;

#[allow(clippy::too_many_lines)]
pub(crate) fn on_error_jvm_buildpack(error: OpenJdkBuildpackError) {
    match error {
        // This mimics the classic behaviour of using download errors as indication for unsupported
        // versions. We want to move off of this mechanism by maintaining a static list of supported
        // versions and their download locations.
        OpenJdkBuildpackError::OpenJdkDownloadError(DownloadError::HttpError(ref http_error))
            if http_error.kind() == ureq::ErrorKind::HTTP => log_error(
                "Unsupported Java version",
                formatdoc! {"
                    Please check your system.properties file to ensure the java.runtime.version
                    is among the list of supported version on the Dev Center:
                    https://devcenter.heroku.com/articles/java-support#supported-java-versions

                    You can also remove the system.properties from your repo to install
                    the default OpenJDK version.

                    If you continue to have trouble, you can open a support ticket here:
                    https://help.heroku.com

                    Thanks,
                    Heroku

                    Details: {error:?}
            ", error = error },
        ),
        OpenJdkBuildpackError::NormalizeVersionStringError(NormalizeVersionStringError::UnknownDistribution(distribution)) => log_error(
            format!("Unsupported distribution: {distribution}"),
            formatdoc! {"
                    Please check your system.properties file to ensure the java.runtime.version
                    string does not contain an unsupported distribution prefix.

                    You can also remove the system.properties from your repo to install
                    the default OpenJDK version.

                    If you continue to have trouble, you can open a support ticket here:
                    https://help.heroku.com

                    Thanks,
                    Heroku
            "},
        ),
        OpenJdkBuildpackError::MetricsAgentDownloadError(error) => log_please_try_again_error(
            "Heroku Metrics Agent download failed",
            "Could not download Heroku Metrics Agent.",
            error,
        ),
        OpenJdkBuildpackError::MetricsAgentSha256ValidationError(sha_256_error) => {
            match sha_256_error {
                ValidateSha256Error::CouldNotObtainSha256(error) =>
                    log_error(
                        "Heroku Metrics Agent download checksum error",
                        formatdoc! {"
                            Heroku Metrics Agent download succeeded, but an error occurred while verifying the
                            SHA256 checksum of the downloaded file.

                            Please try again. If this error persists, please contact us:
                            https://help.heroku.com/

                            Details: {error}
                        ", error = error },
                    ),
                ValidateSha256Error::InvalidChecksum { actual, expected } =>
                    log_error(
                        "Heroku Metrics Agent download checksum error",
                        formatdoc! {"
                            Heroku Metrics Agent download succeeded, but the downloaded file's SHA256
                            checksum {actual} did not match the expected checksum {expected}.

                            Please try again. If this error persists, please contact us:
                            https://help.heroku.com/
                        ", actual = actual, expected = expected },
                    )
            }
        },
        OpenJdkBuildpackError::CannotCreateOpenJdkTempDir(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not create temporary directory for the OpenJDK download due to an unexpected I/O error.",
            error,
        ),
        OpenJdkBuildpackError::ReadVersionStringError(
            ReadSystemPropertiesError::ParseError(error),
        ) => log_error(
            "Invalid system.properties file",
            formatdoc! {"
                Could not parse your application's system.properties file. Please ensure that your
                system.properties file is a valid Java properties file and try again.

                Details: {error}
            ", error = error },
        ),
        OpenJdkBuildpackError::ReadVersionStringError(
            ReadSystemPropertiesError::IoError(error),
        ) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not read your application's system.properties file due to an unexpected I/O error.",
            error,
        ),
        OpenJdkBuildpackError::MissingJdkCertificatesFile => log_please_try_again_error(
            "Missing CA keystore file",
            "The downloaded OpenJDK distribution does not contain a CA keystore file at the expected location.",
            error,
        ),
        OpenJdkBuildpackError::CannotSymlinkUbuntuCertificates(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not symlink the CA keystore file from the stack into the OpenJDK distribution.",
            error,
        ),
        OpenJdkBuildpackError::CannotListJdkOverlayContents(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not list the contents of the application's JDK overlay.",
            error,
        ),
        OpenJdkBuildpackError::CannotCopyJdkOverlayContents(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not copy the contents of the application's JDK overlay.",
            error,
        ),
        OpenJdkBuildpackError::CannotOpenOpenJdkTarball(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not open downloaded OpenJDK tarball file.",
            error,
        ),
        OpenJdkBuildpackError::CannotDecompressOpenJdkTarball(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could decompress downloaded OpenJDK tarball file.",
            error,
        ),
        OpenJdkBuildpackError::OpenJdkDownloadError(error) => log_please_try_again_error(
            "OpenJDK download failed",
            "Could not download OpenJDK distribution.",
            error,
        ),
    }
}
