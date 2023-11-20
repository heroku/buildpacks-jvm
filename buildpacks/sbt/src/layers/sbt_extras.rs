use crate::errors::SbtBuildpackError;
use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
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
            cache: true,
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
            include_bytes!("../../sbt-extras/sbt"),
        )
        .map_err(SbtExtrasLayerError::CouldNotWriteScript)?;

        fs::set_permissions(&sbt_extras_script_path, Permissions::from_mode(0o755))
            .map_err(SbtExtrasLayerError::CouldNotSetPermissions)?;

        let launchers_dir = layer_path.join("launchers");
        fs::create_dir_all(&launchers_dir)
            .map_err(SbtExtrasLayerError::CouldNotCreateLaunchersDir)?;

        LayerResultBuilder::new(GenericMetadata::default())
            .env(
                LayerEnv::new()
                    .chainable_insert(
                        get_layer_env_scope(self.available_at_launch),
                        ModificationBehavior::Delimiter,
                        "SBT_OPTS",
                        " ",
                    )
                    .chainable_insert(
                        get_layer_env_scope(self.available_at_launch),
                        ModificationBehavior::Append,
                        "SBT_OPTS",
                        format!("-sbt-launch-dir {}", launchers_dir.to_string_lossy()),
                    ),
            )
            .build()
    }

    fn existing_layer_strategy(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        _layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        Ok(ExistingLayerStrategy::Keep)
    }
}

fn get_layer_env_scope(available_at_launch: bool) -> Scope {
    if available_at_launch {
        Scope::All
    } else {
        Scope::Build
    }
}

#[derive(Debug)]
pub(crate) enum SbtExtrasLayerError {
    CouldNotWriteScript(std::io::Error),
    CouldNotSetPermissions(std::io::Error),
    CouldNotCreateLaunchersDir(std::io::Error),
}

impl From<SbtExtrasLayerError> for SbtBuildpackError {
    fn from(value: SbtExtrasLayerError) -> Self {
        Self::SbtExtrasLayerError(value)
    }
}
