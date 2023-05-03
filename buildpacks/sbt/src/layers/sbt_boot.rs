use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::buildpack::StackId;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(crate) struct SbtBootLayer {
    pub(crate) for_sbt_version: Version,
    pub(crate) available_at_launch: bool,
}

impl Layer for SbtBootLayer {
    type Buildpack = SbtBuildpack;
    type Metadata = SbtLayerMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: self.available_at_launch,
            cache: true,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        LayerResultBuilder::new(SbtLayerMetadata::current(self, context))
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
                        format!("-Dsbt.boot.directory={}", layer_path.to_string_lossy(),),
                    ),
            )
            .build()
    }

    fn existing_layer_strategy(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        let strategy =
            if layer_data.content_metadata.metadata == SbtLayerMetadata::current(self, context) {
                ExistingLayerStrategy::Keep
            } else {
                ExistingLayerStrategy::Recreate
            };

        Ok(strategy)
    }
}

fn get_layer_env_scope(available_at_launch: bool) -> Scope {
    if available_at_launch {
        Scope::All
    } else {
        Scope::Build
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub(crate) struct SbtLayerMetadata {
    sbt_version: Version,
    layer_version: String,
    stack_id: StackId,
}

const LAYER_VERSION: &str = "v1";

impl SbtLayerMetadata {
    fn current(layer: &SbtBootLayer, context: &BuildContext<SbtBuildpack>) -> Self {
        SbtLayerMetadata {
            sbt_version: layer.for_sbt_version.clone(),
            stack_id: context.stack_id.clone(),
            layer_version: String::from(LAYER_VERSION),
        }
    }
}
