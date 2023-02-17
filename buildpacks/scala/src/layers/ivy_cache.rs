use crate::ScalaBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use std::path::Path;

pub struct IvyCacheLayer;

impl Layer for IvyCacheLayer {
    type Buildpack = ScalaBuildpack;
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
        _: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        LayerResultBuilder::new(GenericMetadata::default())
            .env(create_ivy_layer_env(layer_path))
            .build()
    }

    fn existing_layer_strategy(
        &self,
        _: &BuildContext<Self::Buildpack>,
        _: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        Ok(ExistingLayerStrategy::Keep)
    }
}

fn create_ivy_layer_env(layer_path: &Path) -> LayerEnv {
    LayerEnv::new()
        .chainable_insert(
            Scope::Build,
            ModificationBehavior::Delimiter,
            "SBTX_OPTS",
            " ",
        )
        .chainable_insert(
            Scope::Build,
            ModificationBehavior::Append,
            "SBTX_OPTS",
            shell_words::join(["-ivy", layer_path.display().to_string().as_str()]),
        )
}

#[cfg(test)]
mod ivy_cache_layer_tests {
    use crate::layers::ivy_cache::create_ivy_layer_env;
    use libcnb::layer_env::Scope;
    use std::ffi::OsString;
    use tempfile::tempdir;

    #[test]
    fn create_ivy_layer_env_sets_ivy_flag_in_sbtx_opts() {
        let layer_path = tempdir().unwrap();
        let layer_env = create_ivy_layer_env(layer_path.path());
        let env = layer_env.apply_to_empty(Scope::Build);
        assert_eq!(
            env.get("SBTX_OPTS").unwrap(),
            OsString::from(format!("-ivy {}", layer_path.path().to_string_lossy()))
        );
    }
}
