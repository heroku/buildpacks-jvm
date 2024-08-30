use crate::errors::SbtBuildpackError;
use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{CachedLayerDefinition, InvalidMetadataAction, RestoredLayerAction};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Env;

pub(crate) fn handle_dependency_resolver_home(
    context: &BuildContext<SbtBuildpack>,
    available_at_launch: bool,
    dependency_resolver: DependencyResolver,
    env: &mut Env,
) -> libcnb::Result<(), SbtBuildpackError> {
    let layer_ref = context.cached_layer(
        match dependency_resolver {
            DependencyResolver::Ivy => layer_name!("ivy-home"),
            DependencyResolver::Coursier => layer_name!("coursier-home"),
        },
        CachedLayerDefinition {
            build: false,
            launch: available_at_launch,
            invalid_metadata_action: &|_| InvalidMetadataAction::DeleteLayer,
            restored_layer_action: &|_: &GenericMetadata, _| RestoredLayerAction::KeepLayer,
        },
    )?;

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
                format!(
                    "-D{}={}",
                    match dependency_resolver {
                        DependencyResolver::Ivy => "sbt.ivy.home",
                        DependencyResolver::Coursier => "sbt.coursier.home",
                    },
                    layer_ref.path().to_string_lossy()
                ),
            ),
    )?;

    *env = layer_ref.read_env()?.apply(Scope::Build, env);
    Ok(())
}

#[derive(Copy, Clone)]
pub(crate) enum DependencyResolver {
    Ivy,
    Coursier,
}
