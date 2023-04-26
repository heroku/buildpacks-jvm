use crate::ScalaBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use libherokubuildpack::log::log_info;
use std::path::Path;

pub(crate) struct IvyCacheLayer {
    pub(crate) available_at_launch: Option<bool>,
}

// Ivy is used as the default library management tool up for sbt < 1.3
impl Layer for IvyCacheLayer {
    type Buildpack = ScalaBuildpack;
    type Metadata = GenericMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: self.available_at_launch.unwrap_or_default(),
            cache: true,
        }
    }

    fn create(
        &self,
        _: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        log_info("Creating Ivy cache");
        LayerResultBuilder::new(GenericMetadata::default())
            .env(create_ivy_layer_env(layer_path, self.available_at_launch))
            .build()
    }

    fn existing_layer_strategy(
        &self,
        _: &BuildContext<Self::Buildpack>,
        _: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        log_info("Using existing Ivy cache");
        Ok(ExistingLayerStrategy::Keep)
    }
}

fn create_ivy_layer_env(layer_path: &Path, available_at_launch: Option<bool>) -> LayerEnv {
    // XXX: you may be wondering why JVM_OPTS is used here instead of the more obvious SBT_OPTS
    //      environment variable. due to either my general lack of shell scripting knowledge or a bug
    //      in the sbt-extras script, the SBT_OPTS settings never seem to make it through to the executing
    //      process. everything seem fine until about this point in the sbt-extras script:
    //      - https://github.com/dwijnand/sbt-extras/blob/master/sbt#L541-L565
    //
    //      no matter, setting the JVM_OPTS is just as valid as SBT_OPTS and it seems to be respected
    //      by the sbt-extras script so i'm using that instead:
    //      - https://www.scala-sbt.org/1.x/docs/Command-Line-Reference.html#sbt+JVM+options+and+system+properties
    LayerEnv::new()
        .chainable_insert(
            get_layer_env_scope(available_at_launch),
            ModificationBehavior::Delimiter,
            "JVM_OPTS",
            " ",
        )
        .chainable_insert(
            get_layer_env_scope(available_at_launch),
            ModificationBehavior::Append,
            "JVM_OPTS",
            format!("-Dsbt.ivy.home={}", layer_path.to_string_lossy()),
        )
}

fn get_layer_env_scope(available_at_launch: Option<bool>) -> Scope {
    if available_at_launch.unwrap_or_default() {
        Scope::All
    } else {
        Scope::Build
    }
}

#[cfg(test)]
mod test {
    use crate::layers::ivy_cache::create_ivy_layer_env;
    use libcnb::layer_env::Scope;
    use std::path::Path;

    #[test]
    fn create_ivy_layer_env_sets_ivy_flag_in_sbtx_opts() {
        let layer_path = Path::new("./test_path");
        let layer_env = create_ivy_layer_env(layer_path, None);
        let env = layer_env.apply_to_empty(Scope::Build);
        assert_eq!(env.get("JVM_OPTS").unwrap(), "-Dsbt.ivy.home=./test_path");
    }
}
