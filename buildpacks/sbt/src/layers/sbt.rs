use crate::errors::SbtBuildpackError;
use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::buildpack::StackId;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

pub(crate) struct SbtLayer {
    pub(crate) sbt_version: Version,
    pub(crate) available_at_launch: bool,
}

impl Layer for SbtLayer {
    type Buildpack = SbtBuildpack;
    type Metadata = SbtLayerMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: self.available_at_launch,
            cache: true,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        write_buildpack_plugin(layer_path, &self.sbt_version)?;

        LayerResultBuilder::new(SbtLayerMetadata::current(self, context))
            .env(
                LayerEnv::new()
                    .chainable_insert(
                        get_layer_env_scope(self.available_at_launch),
                        ModificationBehavior::Override,
                        "SBT_HOME",
                        layer_path,
                    )
                    .chainable_insert(
                        get_layer_env_scope(self.available_at_launch),
                        ModificationBehavior::Delimiter,
                        "SBT_OPTS",
                        " ",
                    )
                    .chainable_insert(
                        get_layer_env_scope(self.available_at_launch),
                        ModificationBehavior::Append,
                        "SBT_OPTS",
                        format!(
                            "-sbt-dir {} -sbt-boot {} -sbt-launch-dir {}",
                            sbt_global_dir(layer_path).to_string_lossy(),
                            sbt_boot_dir(layer_path).to_string_lossy(),
                            sbt_launch_dir(layer_path).to_string_lossy(),
                        ),
                    ),
            )
            .build()
    }

    fn existing_layer_strategy(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        let strategy =
            if layer_data.content_metadata.metadata == SbtLayerMetadata::current(self, context) {
                ExistingLayerStrategy::Keep
            } else {
                ExistingLayerStrategy::Recreate
            };

        Ok(strategy)
    }
}

fn get_layer_env_scope(available_at_launch: bool) -> Scope {
    if available_at_launch {
        Scope::All
    } else {
        Scope::Build
    }
}

fn write_buildpack_plugin(
    layer_path: &Path,
    sbt_version: &Version,
) -> Result<(), SbtBuildpackError> {
    let plugin_directory = sbt_global_plugins_dir(layer_path);
    create_dir_all(&plugin_directory).map_err(SbtBuildpackError::CouldNotWriteSbtPlugin)?;

    let contents = get_buildpack_plugin_contents(sbt_version)?;
    write(
        plugin_directory.join("HerokuBuildpackPlugin.scala"),
        contents,
    )
    .map_err(SbtBuildpackError::CouldNotWriteSbtPlugin)?;

    Ok(())
}

fn get_buildpack_plugin_contents(
    sbt_version: &Version,
) -> Result<&'static [u8], SbtBuildpackError> {
    match sbt_version {
        Version { major: 0, .. } => Ok(include_bytes!(
            "../../assets/heroku_buildpack_plugin_sbt_v0.scala"
        )),
        Version { major: 1, .. } => Ok(include_bytes!(
            "../../assets/heroku_buildpack_plugin_sbt_v1.scala"
        )),
        _ => Err(SbtBuildpackError::NoBuildpackPluginAvailable(
            sbt_version.to_string(),
        )),
    }
}

fn sbt_boot_dir(layer_path: &Path) -> PathBuf {
    layer_path.join("boot")
}

fn sbt_global_dir(layer_path: &Path) -> PathBuf {
    layer_path.join("global")
}

fn sbt_global_plugins_dir(layer_path: &Path) -> PathBuf {
    sbt_global_dir(layer_path).join("plugins")
}

fn sbt_launch_dir(layer_path: &Path) -> PathBuf {
    layer_path.join("launch")
}

#[cfg(test)]
mod test {
    use crate::layers::sbt::{sbt_global_plugins_dir, write_buildpack_plugin};
    use semver::Version;
    use tempfile::tempdir;

    #[test]
    fn write_build_plugin_with_sbt_version_0x() {
        let layer_path = tempdir().unwrap();
        let version = Version::parse("0.13.0").unwrap();
        write_buildpack_plugin(layer_path.path(), &version).unwrap();
        assert!(sbt_global_plugins_dir(layer_path.path())
            .join("HerokuBuildpackPlugin.scala")
            .exists());
    }

    #[test]
    fn write_build_plugin_with_sbt_version_1x() {
        let layer_path = tempdir().unwrap();
        let version = Version::parse("1.8.3").unwrap();
        write_buildpack_plugin(layer_path.path(), &version).unwrap();
        assert!(sbt_global_plugins_dir(layer_path.path())
            .join("HerokuBuildpackPlugin.scala")
            .exists());
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub(crate) struct SbtLayerMetadata {
    sbt_version: Version,
    layer_version: String,
    stack_id: StackId,
}

const LAYER_VERSION: &str = "v1";

impl SbtLayerMetadata {
    fn current(layer: &SbtLayer, context: &BuildContext<SbtBuildpack>) -> Self {
        SbtLayerMetadata {
            sbt_version: layer.sbt_version.clone(),
            stack_id: context.stack_id.clone(),
            layer_version: String::from(LAYER_VERSION),
        }
    }
}
