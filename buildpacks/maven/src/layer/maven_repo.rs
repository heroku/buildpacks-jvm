use crate::MavenBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{Layer, LayerResult, LayerResultBuilder};
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
        _context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        let repository_path = layer_path.join(".m2/repository");

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
                        format!("-Dmaven.repo.local={}", &repository_path.to_string_lossy()),
                    ),
            )
            .build()
    }
}
