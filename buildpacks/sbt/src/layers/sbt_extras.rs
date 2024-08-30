use crate::errors::SbtBuildpackError;
use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{
    CachedLayerDefinition, InvalidMetadataAction, LayerState, RestoredLayerAction,
};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Env;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

pub(crate) fn handle_sbt_extras(
    context: &BuildContext<SbtBuildpack>,
    available_at_launch: bool,
    env: &mut Env,
) -> libcnb::Result<(), SbtBuildpackError> {
    let layer_ref = context.cached_layer(
        layer_name!("sbt-extras"),
        CachedLayerDefinition {
            build: false,
            launch: available_at_launch,
            invalid_metadata_action: &|_| InvalidMetadataAction::DeleteLayer,
            restored_layer_action: &|_: &GenericMetadata, _| RestoredLayerAction::KeepLayer,
        },
    )?;

    if let LayerState::Empty { .. } = layer_ref.state {
        let sbt_extras_script_path = layer_ref.path().join("bin").join("sbt");

        if let Some(sbt_extras_script_path_parent) = sbt_extras_script_path.parent() {
            fs::create_dir_all(sbt_extras_script_path_parent).map_err(|error| {
                SbtBuildpackError::SbtExtrasLayerError(SbtExtrasLayerError::CouldNotWriteScript(
                    error,
                ))
            })?;
        }

        fs::write(
            &sbt_extras_script_path,
            include_bytes!("../../sbt-extras/sbt"),
        )
        .map_err(|error| {
            SbtBuildpackError::SbtExtrasLayerError(SbtExtrasLayerError::CouldNotWriteScript(error))
        })?;

        fs::set_permissions(&sbt_extras_script_path, Permissions::from_mode(0o755)).map_err(
            |error| {
                SbtBuildpackError::SbtExtrasLayerError(SbtExtrasLayerError::CouldNotSetPermissions(
                    error,
                ))
            },
        )?;

        let launchers_dir = layer_ref.path().join("launchers");
        fs::create_dir_all(&launchers_dir).map_err(|error| {
            SbtBuildpackError::SbtExtrasLayerError(SbtExtrasLayerError::CouldNotCreateLaunchersDir(
                error,
            ))
        })?;

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
                    format!("-sbt-launch-dir {}", launchers_dir.to_string_lossy()),
                ),
        )?;
    }

    *env = layer_ref.read_env()?.apply(Scope::Build, env);
    Ok(())
}

#[derive(Debug)]
pub(crate) enum SbtExtrasLayerError {
    CouldNotWriteScript(std::io::Error),
    CouldNotSetPermissions(std::io::Error),
    CouldNotCreateLaunchersDir(std::io::Error),
}
