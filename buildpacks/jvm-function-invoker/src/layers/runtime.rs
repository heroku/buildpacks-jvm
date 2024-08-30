use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::{CachedLayerDefinition, InvalidMetadataAction, RestoredLayerAction};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Env;
use libherokubuildpack::digest::sha256;
use libherokubuildpack::download::{download_file, DownloadError};
use libherokubuildpack::log::log_info;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) fn handle_runtime(
    context: &BuildContext<JvmFunctionInvokerBuildpack>,
    env: &mut Env,
) -> libcnb::Result<(), JvmFunctionInvokerBuildpackError> {
    let layer_ref = context.cached_layer(
        layer_name!("runtime"),
        CachedLayerDefinition {
            build: false,
            launch: true,
            invalid_metadata_action: &|_| InvalidMetadataAction::DeleteLayer,
            restored_layer_action: &|metadata: &RuntimeLayerMetadata, _| {
                let sha256_matches = metadata.installed_runtime_sha256
                    == context.buildpack_descriptor.metadata.runtime.sha256;

                if sha256_matches {
                    log_info("Using cached Java function runtime from previous build.");
                    RestoredLayerAction::KeepLayer
                } else {
                    RestoredLayerAction::DeleteLayer
                }
            },
        },
    )?;

    log_info("Starting download of function runtime");

    let runtime_jar_path = layer_ref.path().join("sf-fx-runtime-java.jar");

    download_file(
        &context.buildpack_descriptor.metadata.runtime.url,
        &runtime_jar_path,
    )
    .map_err(RuntimeLayerError::DownloadFailed)?;

    log_info("Function runtime download successful");

    let actual_runtime_jar_sha256 =
        sha256(&runtime_jar_path).map_err(RuntimeLayerError::ChecksumFailed)?;

    if actual_runtime_jar_sha256 == context.buildpack_descriptor.metadata.runtime.sha256 {
        layer_ref.write_metadata(RuntimeLayerMetadata {
            installed_runtime_sha256: actual_runtime_jar_sha256,
        })?;

        layer_ref.write_env(LayerEnv::new().chainable_insert(
            Scope::All,
            ModificationBehavior::Override,
            RUNTIME_JAR_PATH_ENV_VAR_NAME,
            runtime_jar_path,
        ))?;
    } else {
        Err(RuntimeLayerError::ChecksumMismatch(
            actual_runtime_jar_sha256,
        ))?;
    }

    *env = layer_ref.read_env()?.apply(Scope::Build, env);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RuntimeLayerMetadata {
    installed_runtime_sha256: String,
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

impl From<RuntimeLayerError> for libcnb::Error<JvmFunctionInvokerBuildpackError> {
    fn from(value: RuntimeLayerError) -> Self {
        libcnb::Error::BuildpackError(JvmFunctionInvokerBuildpackError::RuntimeLayerError(value))
    }
}

pub(crate) const RUNTIME_JAR_PATH_ENV_VAR_NAME: &str = "JVM_FUNCTION_RUNTIME_JAR_PATH";
