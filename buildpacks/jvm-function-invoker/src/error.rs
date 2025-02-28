use indoc::{formatdoc, indoc};
use libcnb::Error;

use crate::layers::bundle::BundleLayerError;
use crate::layers::opt::OptLayerError;
use crate::layers::runtime::RuntimeLayerError;
use libherokubuildpack::log::log_error;

#[derive(thiserror::Error, Debug)]
pub(crate) enum JvmFunctionInvokerBuildpackError {
    #[error("Opt layer error: {0}")]
    OptLayerError(#[from] OptLayerError),

    #[error("Runtime layer error: {0}")]
    RuntimeLayerError(#[from] RuntimeLayerError),

    #[error("Bundle layer error: {0}")]
    BundleLayerError(#[from] BundleLayerError),
}

impl From<JvmFunctionInvokerBuildpackError> for Error<JvmFunctionInvokerBuildpackError> {
    fn from(error: JvmFunctionInvokerBuildpackError) -> Self {
        Self::BuildpackError(error)
    }
}

pub(crate) fn handle_buildpack_error(error: JvmFunctionInvokerBuildpackError) {
    match error {
        JvmFunctionInvokerBuildpackError::OptLayerError(inner) => match inner {
            OptLayerError::CouldNotWriteRuntimeScript(io_error)
            | OptLayerError::CouldNotSetExecutableBitForRuntimeScript(io_error) => log_error(
                "Unexpected Error",
                formatdoc! {"
                    An error occurred while copying files from the buildpack directory to the container.
                    {io_error}
                ", io_error = io_error},
            ),
        },
        JvmFunctionInvokerBuildpackError::RuntimeLayerError(inner) => match inner {
            RuntimeLayerError::DownloadFailed(download_error) => log_error(
                "Runtime installation failed",
                formatdoc! {"
                        An error occurred while downloading the Java function runtime. In some cases,
                        this happens due to an unstable network connection. Please try again and see
                        if the error resolves itself.

                        {download_error}
                    ", download_error = download_error},
            ),
            RuntimeLayerError::ChecksumFailed(io_error) => log_error(
                "Runtime installation failed",
                formatdoc! {"
                        The integrity of the downloaded Java function runtime could not be verified
                        because an unexpected IO error occurred during the process:
                        {io_error}
                    ", io_error = io_error},
            ),
            RuntimeLayerError::ChecksumMismatch(checksum) => log_error(
                "Runtime installation failed",
                formatdoc! {"
                        The integrity check of the downloaded Java function runtime failed. The
                        downloaded binary has an unexpected SHA256 checksum:
                        {checksum}
                    ", checksum = checksum},
            ),
        },
        JvmFunctionInvokerBuildpackError::BundleLayerError(inner) => match inner {
            BundleLayerError::NoFunctionsFound => log_error(
                "No functions found",
                indoc! {"
                    Your project does not seem to contain any Java functions.
                    The output above might contain information about issues with your function.
                "},
            ),
            BundleLayerError::MultipleFunctionsFound => log_error(
                "Multiple functions found",
                indoc! {"
                        Your project contains multiple Java functions.
                        Currently, only projects that contain exactly one (1) function are supported.
                    "},
            ),
            BundleLayerError::DetectionFailed(exit_code) => log_error(
                "Detection failed",
                formatdoc! {"
                        Function detection failed with internal error \"{exit_code}\"
                    ", exit_code = exit_code },
            ),
            BundleLayerError::UnexpectedDetectionTermination => log_error(
                "Detection failed",
                indoc! {"
                        Function detection was unexpectedly terminated without an exit code.
                    "},
            ),
            BundleLayerError::BundleCommandIoError(io_error) => log_error(
                "Detection failed",
                formatdoc! {"
                        An unexpected IO error occurred during the detect phase:
                        {io_error}
                    ", io_error = io_error},
            ),
            BundleLayerError::CouldNotReadFunctionBundleToml(toml_error) => log_error(
                "Detection failed",
                formatdoc! {"
                        Could not read function bundle metadata after running detection phase:
                        {toml_error}
                    ", toml_error = toml_error},
            ),
            BundleLayerError::FunctionRuntimeNotFound => log_error(
                "Detection failed",
                indoc! {"
                    Could not find function runtime in environment.
                "},
            ),
        },
    }
}
