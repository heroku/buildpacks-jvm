use crate::JvmFunctionInvokerBuildpack;
use crate::error::JvmFunctionInvokerBuildpackError;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::UncachedLayerDefinition;
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::{Env, TomlFileError, read_toml_file};
use libherokubuildpack::log::{log_header, log_info};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

pub(crate) fn handle_bundle(
    context: &BuildContext<JvmFunctionInvokerBuildpack>,
    env: &Env,
) -> libcnb::Result<(), JvmFunctionInvokerBuildpackError> {
    let layer_ref = context.uncached_layer(
        layer_name!("bundle"),
        UncachedLayerDefinition {
            build: false,
            launch: true,
        },
    )?;

    log_header("Detecting function");

    let invoker_jar_path = env
        .get(crate::layers::runtime::RUNTIME_JAR_PATH_ENV_VAR_NAME)
        .ok_or(BundleLayerError::FunctionRuntimeNotFound)?;

    let exit_status = Command::new("java")
        .args(vec![
            "-jar",
            &invoker_jar_path.to_string_lossy(),
            "bundle",
            &context.app_dir.to_string_lossy(),
            &layer_ref.path().to_string_lossy(),
        ])
        .spawn()
        .map_err(BundleLayerError::BundleCommandIoError)?
        .wait()
        .map_err(BundleLayerError::BundleCommandIoError)?;

    match exit_status.code() {
        Some(0) => {
            log_function_metadata(layer_ref.path())?;

            layer_ref.write_env(LayerEnv::new().chainable_insert(
                Scope::All,
                ModificationBehavior::Override,
                FUNCTION_BUNDLE_DIR_ENV_VAR_NAME,
                layer_ref.path(),
            ))?;
        }
        Some(1) => Err(BundleLayerError::NoFunctionsFound)?,
        Some(2) => Err(BundleLayerError::MultipleFunctionsFound)?,
        Some(code) => Err(BundleLayerError::DetectionFailed(code))?,
        None => Err(BundleLayerError::UnexpectedDetectionTermination)?,
    }

    Ok(())
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
pub(crate) enum BundleLayerError {
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

impl From<BundleLayerError> for libcnb::Error<JvmFunctionInvokerBuildpackError> {
    fn from(value: BundleLayerError) -> Self {
        libcnb::Error::BuildpackError(JvmFunctionInvokerBuildpackError::BundleLayerError(value))
    }
}

const FUNCTION_BUNDLE_DIR_ENV_VAR_NAME: &str = "JVM_FUNCTION_BUNDLE_DIR";
