use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{Layer, LayerResult, LayerResultBuilder};

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpack;

pub(crate) struct OptLayer;

impl Layer for OptLayer {
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
        _context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, JvmFunctionInvokerBuildpackError> {
        let layer_bin_dir = layer_path.join("bin");
        let destination = layer_bin_dir.join(JVM_RUNTIME_SCRIPT_NAME);

        fs::create_dir_all(&layer_bin_dir).map_err(OptLayerError::CouldNotWriteRuntimeScript)?;

        fs::write(&destination, include_bytes!("../../opt/jvm-runtime.sh"))
            .map_err(OptLayerError::CouldNotWriteRuntimeScript)?;

        fs::set_permissions(&destination, fs::Permissions::from_mode(0o755))
            .map_err(OptLayerError::CouldNotSetExecutableBitForRuntimeScript)?;

        LayerResultBuilder::new(GenericMetadata::default()).build()
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum OptLayerError {
    #[error("Could not write runtime script to layer: {0}")]
    CouldNotWriteRuntimeScript(std::io::Error),
    #[error("Could not set executable bit on runtime script: {0}")]
    CouldNotSetExecutableBitForRuntimeScript(std::io::Error),
}

pub(crate) const JVM_RUNTIME_SCRIPT_NAME: &str = "jvm-runtime.sh";
