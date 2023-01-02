use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{Layer, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::{read_toml_file, Env, TomlFileError};
use libherokubuildpack::log::{log_header, log_info};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

pub struct BundleLayer {
    pub env: Env,
}

impl Layer for BundleLayer {
    type Buildpack = JvmFunctionInvokerBuildpack;
    type Metadata = GenericMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            launch: true,
            build: false,
            cache: false,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, JvmFunctionInvokerBuildpackError> {
        log_header("Detecting function");

        let invoker_jar_path = self
            .env
            .get(crate::layers::runtime::RUNTIME_JAR_PATH_ENV_VAR_NAME)
            .ok_or(BundleLayerError::FunctionRuntimeNotFound)?;

        let exit_status = Command::new("java")
            .args(vec![
                "-jar",
                &invoker_jar_path.to_string_lossy(),
                "bundle",
                &context.app_dir.to_string_lossy(),
                &layer_path.to_string_lossy(),
            ])
            .spawn()
            .map_err(BundleLayerError::BundleCommandIoError)?
            .wait()
            .map_err(BundleLayerError::BundleCommandIoError)?;

        match exit_status.code() {
            Some(0) => {
                log_function_metadata(layer_path)?;
                LayerResultBuilder::new(GenericMetadata::default())
                    .env(LayerEnv::new().chainable_insert(
                        Scope::All,
                        ModificationBehavior::Override,
                        FUNCTION_BUNDLE_DIR_ENV_VAR_NAME,
                        layer_path,
                    ))
                    .build()
            }
            Some(1) => Err(BundleLayerError::NoFunctionsFound.into()),
            Some(2) => Err(BundleLayerError::MultipleFunctionsFound.into()),
            Some(code) => Err(BundleLayerError::DetectionFailed(code).into()),
            None => Err(BundleLayerError::UnexpectedDetectionTermination.into()),
        }
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
    let bundle_toml_contents: FunctionBundle = read_toml_file(bundle_toml_path)
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
    #[error("Function runtime not found in environment")]
    FunctionRuntimeNotFound,
    #[error("Function detection failed with unexpected exit code {0}")]
    DetectionFailed(i32),
    #[error("Function detection failed with an unexpected termination of the process")]
    UnexpectedDetectionTermination,
    #[error("Function detection failed with an IO error: {0}")]
    BundleCommandIoError(std::io::Error),
    #[error("Could not read function bundle TOML: {0}")]
    CouldNotReadFunctionBundleToml(TomlFileError),
}

pub const FUNCTION_BUNDLE_DIR_ENV_VAR_NAME: &str = "JVM_FUNCTION_BUNDLE_DIR";
