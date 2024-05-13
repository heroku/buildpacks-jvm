use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libherokubuildpack::digest::sha256;
use libherokubuildpack::download::{download_file, DownloadError};
use libherokubuildpack::log::log_info;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub(crate) struct RuntimeLayer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RuntimeLayerMetadata {
    installed_runtime_sha256: String,
}

impl Layer for RuntimeLayer {
    type Buildpack = JvmFunctionInvokerBuildpack;
    type Metadata = RuntimeLayerMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            launch: true,
            build: false,
            cache: true,
        }
    }

    fn create(
        &mut self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, JvmFunctionInvokerBuildpackError> {
        log_info("Starting download of function runtime");

        let runtime_jar_path = layer_path.join("sf-fx-runtime-java.jar");

        download_file(
            &context.buildpack_descriptor.metadata.runtime.url,
            &runtime_jar_path,
        )
        .map_err(RuntimeLayerError::DownloadFailed)?;

        log_info("Function runtime download successful");

        let actual_runtime_jar_sha256 =
            sha256(&runtime_jar_path).map_err(RuntimeLayerError::ChecksumFailed)?;

        if actual_runtime_jar_sha256 == context.buildpack_descriptor.metadata.runtime.sha256 {
            LayerResultBuilder::new(RuntimeLayerMetadata {
                installed_runtime_sha256: actual_runtime_jar_sha256,
            })
            .env(LayerEnv::new().chainable_insert(
                Scope::All,
                ModificationBehavior::Override,
                RUNTIME_JAR_PATH_ENV_VAR_NAME,
                runtime_jar_path,
            ))
            .build()
        } else {
            Err(RuntimeLayerError::ChecksumMismatch(actual_runtime_jar_sha256).into())
        }
    }

    fn existing_layer_strategy(
        &mut self,
        context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, JvmFunctionInvokerBuildpackError> {
        let sha256_matches = layer_data
            .content_metadata
            .metadata
            .installed_runtime_sha256
            == context.buildpack_descriptor.metadata.runtime.sha256;

        if sha256_matches {
            log_info("Using cached Java function runtime from previous build.");
            Ok(ExistingLayerStrategy::Keep)
        } else {
            Ok(ExistingLayerStrategy::Recreate)
        }
    }
}

#[derive(Error, Debug)]
pub(crate) enum RuntimeLayerError {
    #[error("Could not download runtime JAR: {0}")]
    DownloadFailed(DownloadError),

    #[error("Could not obtain checksum for runtime JAR: {0}")]
    ChecksumFailed(std::io::Error),

    #[error("Checksum validation of runtime JAR failed! Checksum was: {0}")]
    ChecksumMismatch(String),
}

pub(crate) const RUNTIME_JAR_PATH_ENV_VAR_NAME: &str = "JVM_FUNCTION_RUNTIME_JAR_PATH";
