use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use libcnb::data::layer_content_metadata::LayerContentMetadata;
use libcnb::layer_lifecycle::LayerLifecycle;
use libcnb::{BuildContext, GenericMetadata, GenericPlatform};

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;

pub struct OptLayerLifecycle {}

#[cfg(target_family = "unix")]
impl
    LayerLifecycle<
        GenericPlatform,
        JvmFunctionInvokerBuildpackMetadata,
        GenericMetadata,
        PathBuf,
        JvmFunctionInvokerBuildpackError,
    > for OptLayerLifecycle
{
    fn create(
        &self,
        path: &Path,
        context: &BuildContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
    ) -> Result<LayerContentMetadata<GenericMetadata>, JvmFunctionInvokerBuildpackError> {
        let source = context.buildpack_dir.join("opt").join("run.sh");
        let destination = path.join("run.sh");

        fs::copy(&source, &destination).map_err(OptLayerError::CouldNotCopyRunSh)?;

        fs::set_permissions(&destination, fs::Permissions::from_mode(0o755))
            .map_err(OptLayerError::CouldNotSetExecutableBitForRunSh)?;

        Ok(LayerContentMetadata::default().launch(true))
    }

    fn layer_lifecycle_data(
        &self,
        path: &Path,
        _layer_content_metadata: LayerContentMetadata<GenericMetadata>,
    ) -> Result<PathBuf, JvmFunctionInvokerBuildpackError> {
        Ok(path.join("run.sh"))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum OptLayerError {
    #[error("Could not copy run.sh to layer: {0}")]
    CouldNotCopyRunSh(std::io::Error),
    #[error("Could not set executable bit on run.sh: {0}")]
    CouldNotSetExecutableBitForRunSh(std::io::Error),
}
