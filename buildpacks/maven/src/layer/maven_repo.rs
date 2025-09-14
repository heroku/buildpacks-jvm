use crate::{MavenBuildpack, MavenBuildpackError};
use libcnb::Env;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{CachedLayerDefinition, InvalidMetadataAction, RestoredLayerAction};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};

pub(crate) fn handle_maven_repository_layer(
    context: &BuildContext<MavenBuildpack>,
    env: &mut Env,
) -> libcnb::Result<(), MavenBuildpackError> {
    let layer_ref = context.cached_layer(
        layer_name!("repository"),
        CachedLayerDefinition {
            build: false,
            launch: false,
            invalid_metadata_action: &|_| InvalidMetadataAction::DeleteLayer,
            restored_layer_action: &|_: &GenericMetadata, _| RestoredLayerAction::KeepLayer,
        },
    )?;

    layer_ref.write_env(
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
                format!("-Dmaven.repo.local={}", &layer_ref.path().to_string_lossy()),
            ),
    )?;

    *env = layer_ref.read_env()?.apply(Scope::Build, env);
    Ok(())
}
