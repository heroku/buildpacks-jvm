use crate::MavenBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use std::path::Path;

pub struct MavenRepositoryLayer;

impl Layer for MavenRepositoryLayer {
    type Buildpack = MavenBuildpack;
    type Metadata = GenericMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: false,
            cache: true,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        LayerResultBuilder::new(GenericMetadata::default())
            .env(
                LayerEnv::new()
                    .chainable_insert(
                        Scope::Build,
                        ModificationBehavior::Delimiter,
                        "MAVEN_OPTS",
                        " ",
                    )
                    .chainable_insert(
                        Scope::Build,
                        ModificationBehavior::Append,
                        "MAVEN_OPTS",
                        format!(
                            "-Dmaven.repo.local={} -Duser.home={}",
                            &layer_path.to_string_lossy(),
                            context.app_dir.to_string_lossy()
                        ),
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
