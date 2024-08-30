use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::UncachedLayerDefinition;
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub(crate) fn handle_opt(
    context: &BuildContext<JvmFunctionInvokerBuildpack>,
) -> libcnb::Result<(), JvmFunctionInvokerBuildpackError> {
    let layer_ref = context.uncached_layer(
        layer_name!("opt"),
        UncachedLayerDefinition {
            build: false,
            launch: true,
        },
    )?;

    let layer_bin_dir = layer_ref.path().join("bin");
    let destination = layer_bin_dir.join(JVM_RUNTIME_SCRIPT_NAME);

    fs::create_dir_all(&layer_bin_dir).map_err(OptLayerError::CouldNotWriteRuntimeScript)?;

    fs::write(&destination, include_bytes!("../../opt/jvm-runtime.sh"))
        .map_err(OptLayerError::CouldNotWriteRuntimeScript)?;

    fs::set_permissions(&destination, fs::Permissions::from_mode(0o755))
        .map_err(OptLayerError::CouldNotSetExecutableBitForRuntimeScript)?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum OptLayerError {
    #[error("Could not write runtime script to layer: {0}")]
    CouldNotWriteRuntimeScript(std::io::Error),
    #[error("Could not set executable bit on runtime script: {0}")]
    CouldNotSetExecutableBitForRuntimeScript(std::io::Error),
}

impl From<OptLayerError> for libcnb::Error<JvmFunctionInvokerBuildpackError> {
    fn from(value: OptLayerError) -> Self {
        libcnb::Error::BuildpackError(JvmFunctionInvokerBuildpackError::OptLayerError(value))
    }
}

pub(crate) const JVM_RUNTIME_SCRIPT_NAME: &str = "jvm-runtime.sh";
