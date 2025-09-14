use crate::{GRADLE_TASK_NAME_HEROKU_START_DAEMON, GradleBuildpack, GradleBuildpackError};
use indoc::{formatdoc, indoc};
use libcnb::Env;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{
    CachedLayerDefinition, InvalidMetadataAction, LayerState, RestoredLayerAction,
};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use std::fs;

pub(crate) fn handle_gradle_home_layer(
    context: &BuildContext<GradleBuildpack>,
    env: &mut Env,
) -> libcnb::Result<(), GradleBuildpackError> {
    let layer_ref = context.cached_layer(
        layer_name!("home"),
        CachedLayerDefinition {
            build: true,
            launch: false,
            invalid_metadata_action: &|_| InvalidMetadataAction::DeleteLayer,
            restored_layer_action: &|_: &GenericMetadata, _| RestoredLayerAction::KeepLayer,
        },
    )?;

    match layer_ref.state {
        LayerState::Restored { .. } => {
            // Remove daemon metadata from the cached directory. Among other things, it contains a list
            // of PIDs from previous runs that will clutter up the output and aren't meaningful with
            // containerized builds anyway.
            let daemon_dir_path = layer_ref.path().join("daemon");
            if daemon_dir_path.is_dir() {
                // We explicitly ignore potential errors since not being able to remove this directory
                // should not fail the build as it's mostly for output cosmetics only.
                let _ignored_result = fs::remove_dir_all(daemon_dir_path);
            }
        }
        LayerState::Empty { .. } => {
            // https://docs.gradle.org/8.3/userguide/build_environment.html#sec:gradle_configuration_properties
            fs::write(
                layer_ref.path().join("gradle.properties"),
                indoc! {"
                org.gradle.welcome=never
                org.gradle.caching=true
            "},
            )
            .map_err(GradleBuildpackError::WriteGradlePropertiesError)?;

            // We're adding this empty task to all projects to ensure we have a task we can run when
            // we start the Gradle daemon that doesn't side effect or output anything to the console.
            // https://docs.gradle.org/8.3/userguide/init_scripts.html
            fs::write(
                layer_ref.path().join("init.gradle.kts"),
                formatdoc! {"
                allprojects {{
                    tasks.register(\"{task_name}\")
                }}",
                    task_name = GRADLE_TASK_NAME_HEROKU_START_DAEMON
                },
            )
            .map_err(GradleBuildpackError::WriteGradleInitScriptError)?;

            layer_ref.write_env(LayerEnv::new().chainable_insert(
                Scope::All,
                ModificationBehavior::Override,
                "GRADLE_USER_HOME",
                layer_ref.path(),
            ))?;
        }
    }

    *env = layer_ref.read_env()?.apply(Scope::Build, env);
    Ok(())
}
