use crate::errors::SbtBuildpackError;
use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{Layer, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use std::fs;
use std::path::Path;

pub(crate) struct SbtGlobalLayer {
    pub(crate) available_at_launch: bool,
}

impl Layer for SbtGlobalLayer {
    type Buildpack = SbtBuildpack;
    type Metadata = GenericMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: self.available_at_launch,
            cache: false,
        }
    }

    fn create(
        &mut self,
        _context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        let plugin_path = layer_path
            .join("plugins")
            .join("HerokuBuildpackPlugin.scala");

        if let Some(plugin_path_parent) = plugin_path.parent() {
            fs::create_dir_all(plugin_path_parent)
                .map_err(SbtGlobalLayerError::CouldNotWritePlugin)?;
        }

        fs::write(
            plugin_path,
            include_bytes!("../../sbt-plugins/buildpack-plugin-1.x.scala"),
        )
        .map_err(SbtGlobalLayerError::CouldNotWritePlugin)?;

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
                        // See: https://www.scala-sbt.org/1.x/docs/Command-Line-Reference.html
                        format!("-Dsbt.global.base={}", layer_path.to_string_lossy()),
                    ),
            )
            .build()
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
pub(crate) enum SbtGlobalLayerError {
    CouldNotWritePlugin(std::io::Error),
}

impl From<SbtGlobalLayerError> for SbtBuildpackError {
    fn from(value: SbtGlobalLayerError) -> Self {
        SbtBuildpackError::SbtGlobalLayerError(value)
    }
}
