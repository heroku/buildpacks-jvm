use crate::errors::SbtBuildpackError;
use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::{CachedLayerDefinition, InvalidMetadataAction, RestoredLayerAction};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Env;
use semver::Version;
use serde::{Deserialize, Serialize};

pub(crate) fn handle_sbt_boot(
    context: &BuildContext<SbtBuildpack>,
    version: Version,
    available_at_launch: bool,
    env: &mut Env,
) -> libcnb::Result<(), SbtBuildpackError> {
    let layer_ref = context.cached_layer(
        layer_name!("sbt-boot"),
        CachedLayerDefinition {
            build: true,
            launch: available_at_launch,
            invalid_metadata_action: &|_| InvalidMetadataAction::DeleteLayer,
            restored_layer_action: &|metadata: &SbtLayerMetadata, _| {
                if metadata == &SbtLayerMetadata::current(version.clone()) {
                    RestoredLayerAction::KeepLayer
                } else {
                    RestoredLayerAction::DeleteLayer
                }
            },
        },
    )?;

    layer_ref.write_metadata(SbtLayerMetadata::current(version))?;

    let env_scope = if available_at_launch {
        Scope::All
    } else {
        Scope::Build
    };

    layer_ref.write_env(
        LayerEnv::new()
            .chainable_insert(
                env_scope.clone(),
                ModificationBehavior::Delimiter,
                "SBT_OPTS",
                " ",
            )
            .chainable_insert(
                env_scope,
                ModificationBehavior::Append,
                "SBT_OPTS",
                // See: https://www.scala-sbt.org/1.x/docs/Command-Line-Reference.html
                format!(
                    "-Dsbt.boot.directory={}",
                    layer_ref.path().to_string_lossy(),
                ),
            ),
    )?;

    *env = layer_ref.read_env()?.apply(Scope::Build, env);
    Ok(())
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub(crate) struct SbtLayerMetadata {
    sbt_version: Version,
    layer_version: String,
}

const LAYER_VERSION: &str = "v1";

impl SbtLayerMetadata {
    fn current(sbt_version: Version) -> Self {
        SbtLayerMetadata {
            sbt_version,
            layer_version: String::from(LAYER_VERSION),
        }
    }
}
