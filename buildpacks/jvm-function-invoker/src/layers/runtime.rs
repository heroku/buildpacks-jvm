use std::path::{Path, PathBuf};

use libcnb::data::layer_content_metadata::LayerContentMetadata;
use libcnb::layer_lifecycle::{LayerLifecycle, ValidateResult};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;
use libcnb::{BuildContext, GenericPlatform};
use libherokubuildpack::{log_header, log_info, DownloadError, download_file, sha256};

pub struct RuntimeLayerLifecycle {}

impl
    LayerLifecycle<
        GenericPlatform,
        JvmFunctionInvokerBuildpackMetadata,
        RuntimeLayerMetadata,
        PathBuf,
        JvmFunctionInvokerBuildpackError,
    > for RuntimeLayerLifecycle
{
    fn create(
        &self,
        path: &Path,
        context: &BuildContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
    ) -> Result<LayerContentMetadata<RuntimeLayerMetadata>, JvmFunctionInvokerBuildpackError> {
        let runtime_jar_path = path.join("sf-fx-runtime-java.jar");

        log_info("Starting download of function runtime");

        download_file(
            &context.buildpack_descriptor.metadata.runtime.url,
            &runtime_jar_path,
        )
        .map_err(RuntimeLayerError::RuntimeDownloadFailed)?;

        log_info("Function runtime download successful");

        let actual_runtime_jar_sha256 =
            sha256(&runtime_jar_path).map_err(RuntimeLayerError::RuntimeChecksumFailed)?;

        if actual_runtime_jar_sha256 == context.buildpack_descriptor.metadata.runtime.sha256 {
            Ok(LayerContentMetadata::default()
                .launch(true)
                .cache(true)
                .metadata(RuntimeLayerMetadata {
                    installed_runtime_sha256: actual_runtime_jar_sha256,
                }))
        } else {
            Err(RuntimeLayerError::RuntimeChecksumMismatch(actual_runtime_jar_sha256).into())
        }
    }

    fn validate(
        &self,
        _path: &Path,
        layer_content_metadata: &LayerContentMetadata<RuntimeLayerMetadata>,
        build_context: &BuildContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
    ) -> ValidateResult {
        if layer_content_metadata.metadata.installed_runtime_sha256
            == build_context.buildpack_descriptor.metadata.runtime.sha256
        {
            ValidateResult::KeepLayer
        } else {
            ValidateResult::RecreateLayer
        }
    }

    fn layer_lifecycle_data(
        &self,
        path: &Path,
        _layer_content_metadata: LayerContentMetadata<RuntimeLayerMetadata>,
    ) -> Result<PathBuf, JvmFunctionInvokerBuildpackError> {
        Ok(path.join("sf-fx-runtime-java.jar"))
    }

    fn on_lifecycle_start(&self) {
        log_header("Installing Java function runtime");
    }

    fn on_keep(&self) {
        log_info("Using cached Java function runtime from previous build.")
    }

    fn on_lifecycle_end(&self) {
        log_info("Function runtime installation successful");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeLayerMetadata {
    installed_runtime_sha256: String,
}

#[derive(Error, Debug)]
pub enum RuntimeLayerError {
    #[error("Could not download runtime JAR: {0}")]
    RuntimeDownloadFailed(DownloadError),

    #[error("Could not obtain checksum for runtime JAR: {0}")]
    RuntimeChecksumFailed(std::io::Error),

    #[error("Checksum validation of runtime JAR failed! Checksum was: {0}")]
    RuntimeChecksumMismatch(String),
}
