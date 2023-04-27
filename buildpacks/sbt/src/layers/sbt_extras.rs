use crate::errors::SbtBuildpackError;
use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{Layer, LayerResult, LayerResultBuilder};
use libcnb::Buildpack;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub(crate) struct SbtExtrasLayer {
    pub(crate) available_at_launch: bool,
}

impl Layer for SbtExtrasLayer {
    type Buildpack = SbtBuildpack;
    type Metadata = GenericMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            launch: self.available_at_launch,
            build: true,
            cache: false,
        }
    }

    fn create(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        let sbt_extras_script_path = layer_path.join("bin").join("sbt");

        if let Some(sbt_extras_script_path_parent) = sbt_extras_script_path.parent() {
            fs::create_dir_all(sbt_extras_script_path_parent)
                .map_err(SbtExtrasLayerError::CouldNotWriteScript)?;
        }

        fs::write(
            &sbt_extras_script_path,
            include_bytes!("../../assets/sbt-extras.sh"),
        )
        .map_err(SbtExtrasLayerError::CouldNotWriteScript)?;

        fs::set_permissions(&sbt_extras_script_path, Permissions::from_mode(0o755))
            .map_err(SbtExtrasLayerError::CouldNotSetPermissions)?;

        LayerResultBuilder::new(GenericMetadata::default()).build()
    }
}

#[derive(Debug)]
pub(crate) enum SbtExtrasLayerError {
    CouldNotWriteScript(std::io::Error),
    CouldNotSetPermissions(std::io::Error),
}

impl From<SbtExtrasLayerError> for SbtBuildpackError {
    fn from(value: SbtExtrasLayerError) -> Self {
        Self::SbtExtrasLayerError(value)
    }
}
