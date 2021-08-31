use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

use libcnb::data::layer_content_metadata::LayerContentMetadata;
use libcnb::layer_lifecycle::LayerLifecycle;
use libcnb::{read_toml_file, TomlFileError};
use libcnb::{BuildContext, GenericMetadata, GenericPlatform};
use serde::Deserialize;
use thiserror::Error;

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;
use libherokubuildpack::{log_header, log_info};

pub struct BundleLayerLifecycle {
    pub invoker_jar_path: PathBuf,
}

impl
    LayerLifecycle<
        GenericPlatform,
        JvmFunctionInvokerBuildpackMetadata,
        GenericMetadata,
        PathBuf,
        JvmFunctionInvokerBuildpackError,
    > for BundleLayerLifecycle
{
    fn create(
        &self,
        path: &Path,
        context: &BuildContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
    ) -> Result<LayerContentMetadata<GenericMetadata>, JvmFunctionInvokerBuildpackError> {
        let exit_status = Command::new("java")
            .args(vec![
                OsStr::new("-jar"),
                self.invoker_jar_path.as_os_str(),
                OsStr::new("bundle"),
                context.app_dir.as_os_str(),
                path.as_os_str(),
            ])
            .spawn()
            .map_err(BundleLayerError::BundleCommandIoError)?
            .wait()
            .map_err(BundleLayerError::BundleCommandIoError)?;

        match exit_status.code() {
            Some(0) => {
                log_function_metadata(&path)?;

                Ok(LayerContentMetadata::default()
                    .launch(true)
                    .build(false)
                    .cache(false))
            }
            Some(1) => Err(BundleLayerError::NoFunctionsFound.into()),
            Some(2) => Err(BundleLayerError::MultipleFunctionsFound.into()),
            Some(code) => Err(BundleLayerError::DetectionFailed(code).into()),
            None => Err(BundleLayerError::UnexpectedDetectionTermination.into()),
        }
    }

    fn layer_lifecycle_data(
        &self,
        path: &Path,
        _layer_content_metadata: LayerContentMetadata<GenericMetadata>,
    ) -> Result<PathBuf, JvmFunctionInvokerBuildpackError> {
        Ok(path.to_path_buf())
    }

    fn on_lifecycle_start(&self) {
        log_header("Detecting function");
    }
}

fn log_function_metadata(bundle_dir: impl AsRef<Path>) -> Result<(), BundleLayerError> {
    #[derive(Deserialize, Debug)]
    struct FunctionBundle {
        function: Function,
    }

    #[derive(Deserialize, Debug)]
    struct Function {
        class: String,
        payload_class: String,
        return_class: String,
    }

    let bundle_toml_path = bundle_dir.as_ref().join("function-bundle.toml");
    let bundle_toml_contents: FunctionBundle = read_toml_file(&bundle_toml_path)
        .map_err(BundleLayerError::CouldNotReadFunctionBundleToml)?;

    log_header(format!(
        "Detected Function: {}",
        &bundle_toml_contents.function.class
    ));

    log_info(format!(
        "Payload type: {}",
        bundle_toml_contents.function.payload_class
    ));

    log_info(format!(
        "Return type: {}",
        bundle_toml_contents.function.return_class
    ));

    Ok(())
}

#[derive(Error, Debug)]
pub enum BundleLayerError {
    #[error("Project does not contain any valid functions")]
    NoFunctionsFound,
    #[error("Project contains multiple functions")]
    MultipleFunctionsFound,
    #[error("Function detection failed with unexpected exit code {0}")]
    DetectionFailed(i32),
    #[error("Function detection failed with an unexpected termination of the process")]
    UnexpectedDetectionTermination,
    #[error("Function detection failed with an IO error: {0}")]
    BundleCommandIoError(std::io::Error),
    #[error("Could not read function bundle TOML: {0}")]
    CouldNotReadFunctionBundleToml(TomlFileError),
}
