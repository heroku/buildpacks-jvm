use crate::{GradleBuildpack, GradleBuildpackError, GRADLE_TASK_NAME_HEROKU_START_DAEMON};
use indoc::{formatdoc, indoc};
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use std::fs;
use std::path::Path;

pub(crate) struct GradleHomeLayer;

impl Layer for GradleHomeLayer {
    type Buildpack = GradleBuildpack;
    type Metadata = GenericMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            launch: true,
            build: true,
            cache: true,
        }
    }

    fn create(
        &mut self,
        _context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        // https://docs.gradle.org/8.3/userguide/build_environment.html#sec:gradle_configuration_properties
        fs::write(
            layer_path.join("gradle.properties"),
            indoc! {"
                org.gradle.welcome=never
                org.gradle.caching=true
            "},
        )
        .map_err(GradleBuildpackError::WriteGradlePropertiesError)?;

        // We're adding this empty task to all projects to ensure we have a task we can run when
        // we start the Gradle daemon that doesn't side-effect or output anything to the console.
        // https://docs.gradle.org/8.3/userguide/init_scripts.html
        fs::write(
            layer_path.join("init.gradle.kts"),
            formatdoc! {"
                allprojects {{
                    tasks.register(\"{task_name}\")
                }}",
                task_name = GRADLE_TASK_NAME_HEROKU_START_DAEMON
            },
        )
        .map_err(GradleBuildpackError::WriteGradleInitScriptError)?;

        LayerResultBuilder::new(None)
            .env(LayerEnv::new().chainable_insert(
                Scope::All,
                ModificationBehavior::Override,
                "GRADLE_USER_HOME",
                layer_path,
            ))
            .build()
    }

    fn existing_layer_strategy(
        &mut self,
        _context: &BuildContext<Self::Buildpack>,
        _layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        Ok(ExistingLayerStrategy::Update)
    }

    fn update(
        &mut self,
        _context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        // Remove daemon metadata from the cached directory. Among other things, it contains a list
        // of PIDs from previous runs that will clutter up the output and aren't meaningful with
        // containerized builds anyway.
        let daemon_dir_path = layer_data.path.join("daemon");
        if daemon_dir_path.is_dir() {
            // We explicitly ignore potential errors since not being able to remove this directory
            // should not fail the build as it's mostly for output cosmetics only.
            let _ignored_result = fs::remove_dir_all(daemon_dir_path);
        }

        LayerResultBuilder::new(layer_data.content_metadata.metadata.clone())
            .env(layer_data.env.clone())
            .build()
    }
}
