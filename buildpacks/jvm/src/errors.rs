use crate::openjdk_artifact::{
    HerokuOpenJdkVersionRequirement, OpenJdkArtifactRequirementParseError,
};
use crate::version_resolver::VersionResolveError;
use crate::OpenJdkBuildpackError;
use buildpacks_jvm_shared::log::{log_please_try_again, log_please_try_again_error};
use buildpacks_jvm_shared::system_properties::ReadSystemPropertiesError;
use indoc::formatdoc;
use libherokubuildpack::log::log_error;

#[allow(clippy::too_many_lines)]
pub(crate) fn on_error_jvm_buildpack(error: OpenJdkBuildpackError) {
    match error {
        OpenJdkBuildpackError::CannotCreateOpenJdkTempDir(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not create temporary directory for the OpenJDK download due to an unexpected I/O error.",
            error,
        ),
        OpenJdkBuildpackError::ReadSystemPropertiesError(
            ReadSystemPropertiesError::ParseError(error),
        ) => log_error(
            "Invalid system.properties file",
            formatdoc! {"
                Could not parse your application's system.properties file. Please ensure that your
                system.properties file is a valid Java properties file and try again.

                Details: {error}
            ", error = error },
        ),
        OpenJdkBuildpackError::ReadSystemPropertiesError(
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
        OpenJdkBuildpackError::CannotReadOpenJdkTarball(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not read downloaded OpenJDK tarball file.",
            error,
        ),
        OpenJdkBuildpackError::CannotDecompressOpenJdkTarball(error) => log_please_try_again_error(
            "Unexpected IO error",
            "Could not decompress downloaded OpenJDK tarball file.",
            error,
        ),
        OpenJdkBuildpackError::OpenJdkDownloadError(error) => log_please_try_again_error(
            "OpenJDK download failed",
            "Could not download OpenJDK distribution.",
            error,
        ),
        OpenJdkBuildpackError::UnsupportedOpenJdkVersion(artifact_requirement) => {
            match artifact_requirement.version {
                HerokuOpenJdkVersionRequirement::Major(major_version) => log_error(
                    "Unsupported OpenJDK version",
                    formatdoc! {"
                        The OpenJDK major version {major_version} you specified in your system.properties file is not supported.
                        Please specify a supported major version in your system.properties file.
                    ", major_version = major_version },
                ),
                HerokuOpenJdkVersionRequirement::Specific(version) => log_error(
                    "Unsupported OpenJDK version",
                    formatdoc! {"
                        The OpenJDK version {version} you specified in your system.properties file is not supported.
                        Please specify a supported version in your system.properties file.

                        We recommend specifying only the major version in system.properties.
                        This will cause the buildpack to always install the latest version of the chosen major version.
                    ", version = version },
                )
            }
        },
        OpenJdkBuildpackError::ParseInventoryError(error) => log_error(
            "Invalid Inventory File",
            formatdoc! {"
                The inventory of OpenJDK distributions could not be parsed. This error should
                never occur to users of this buildpack and is almost always a buildpack bug.

                If you see this error, please file an issue:
                https://github.com/heroku/buildpacks-jvm/issues/new

                Details: {error}
            ", error = error },
        ),
        OpenJdkBuildpackError::OpenJdkTarballChecksumError { expected, actual } => log_please_try_again(
            "Corrupted OpenJDK download",
            formatdoc! {"
                The validation of the downloaded OpenJDK distribution failed due to a checksum mismatch.

                Expected: {expected}
                Actual: {actual}
            ", expected = hex::encode(expected), actual = hex::encode(actual) }
        ),
        OpenJdkBuildpackError::ResolveVersionError(VersionResolveError::OpenJdkArtifactRequirementParseError(OpenJdkArtifactRequirementParseError::UnknownDistribution(distribution))) => log_error(
            format!("Unsupported distribution: {distribution}"),
            formatdoc! {"
                    Please check your system.properties file to ensure the java.runtime.version
                    string does not contain an unsupported distribution prefix.

                    You can also remove the system.properties file from your application to install
                    the default OpenJDK version.

                    Thanks,
                    Heroku
            "}),
        OpenJdkBuildpackError::ResolveVersionError(VersionResolveError::ReadSystemPropertiesError(error)) => {
            log_error(
                "Invalid OpenJDK version selector",
                formatdoc! {"
            The OpenJDK version selector you specified in your system.properties file is invalid.
            Please specify a valid version selector in your system.properties file.

            Details: {error:?}
        ", error = error },
            );
        }
        OpenJdkBuildpackError::ResolveVersionError(VersionResolveError::OpenJdkArtifactRequirementParseError(OpenJdkArtifactRequirementParseError::OpenJdkVersionParseError(_))) => {}
    }
}
