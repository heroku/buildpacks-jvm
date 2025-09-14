use crate::SbtBuildpack;
use crate::errors::SbtBuildpackError;
use libcnb::Env;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::UncachedLayerDefinition;
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use std::fs;

pub(crate) fn handle_sbt_global(
    context: &BuildContext<SbtBuildpack>,
    available_at_launch: bool,
    env: &mut Env,
) -> libcnb::Result<(), SbtBuildpackError> {
    let layer_ref = context.uncached_layer(
        layer_name!("sbt-global"),
        UncachedLayerDefinition {
            build: false,
            launch: available_at_launch,
        },
    )?;

    let plugin_path = layer_ref
        .path()
        .join("plugins")
        .join("HerokuBuildpackPlugin.scala");

    if let Some(plugin_path_parent) = plugin_path.parent() {
        fs::create_dir_all(plugin_path_parent).map_err(|error| {
            SbtBuildpackError::SbtGlobalLayerError(SbtGlobalLayerError::CouldNotWritePlugin(error))
        })?;
    }

    fs::write(
        plugin_path,
        include_bytes!("../../sbt-plugins/buildpack-plugin-1.x.scala"),
    )
    .map_err(|error| {
        SbtBuildpackError::SbtGlobalLayerError(SbtGlobalLayerError::CouldNotWritePlugin(error))
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
                // See: https://www.scala-sbt.org/1.x/docs/Command-Line-Reference.html
                format!("-Dsbt.global.base={}", layer_ref.path().to_string_lossy()),
            ),
    )?;

    *env = layer_ref.read_env()?.apply(Scope::Build, env);
    Ok(())
}

#[derive(Debug)]
pub(crate) enum SbtGlobalLayerError {
    CouldNotWritePlugin(std::io::Error),
}
