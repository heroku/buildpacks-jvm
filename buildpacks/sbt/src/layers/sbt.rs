use crate::errors::SbtBuildpackError;
use crate::SbtBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::buildpack::StackId;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::{Buildpack, Env};
use libherokubuildpack::log::log_info;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, set_permissions, write, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

pub(crate) struct SbtLayer {
    pub(crate) sbt_version: Version,
    pub(crate) env: Env,
    pub(crate) available_at_launch: Option<bool>,
}

impl Layer for SbtLayer {
    type Buildpack = SbtBuildpack;
    type Metadata = SbtLayerMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: self.available_at_launch.unwrap_or_default(),
            cache: true,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        log_info(format!("Setting up sbt {}", self.sbt_version));

        write_sbt_extras_to_layer(layer_path)?;
        write_sbt_wrapper_to_layer(layer_path)?;

        let layer_env = create_sbt_layer_env(layer_path, self.available_at_launch);
        let env = layer_env.apply(Scope::Build, &self.env);

        install_sbt(&context.app_dir, layer_path, &env)?;
        write_buildpack_plugin(layer_path, &self.sbt_version)?;

        LayerResultBuilder::new(SbtLayerMetadata::current(self, context))
            .env(layer_env)
            .build()
    }

    fn existing_layer_strategy(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        if layer_data.content_metadata.metadata == SbtLayerMetadata::current(self, context) {
            log_info(format!("Reusing sbt {}", self.sbt_version));
            return Ok(ExistingLayerStrategy::Keep);
        }
        Ok(ExistingLayerStrategy::Recreate)
    }
}

fn install_sbt(
    app_dir: &PathBuf,
    layer_path: &Path,
    env: &Env,
) -> Result<ExitStatus, SbtBuildpackError> {
    Command::new(sbt_path(layer_path))
        .current_dir(app_dir)
        .args(["sbtVersion"])
        .envs(env)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(SbtBuildpackError::SbtInstallIoError)
        .and_then(|exit_status| {
            if exit_status.success() {
                Ok(exit_status)
            } else {
                Err(SbtBuildpackError::SbtInstallUnexpectedExitCode(exit_status))
            }
        })
}

fn create_sbt_layer_env(layer_path: &Path, available_at_launch: Option<bool>) -> LayerEnv {
    LayerEnv::new()
        .chainable_insert(
            get_layer_env_scope(available_at_launch),
            ModificationBehavior::Override,
            "SBT_HOME",
            layer_path,
        )
        .chainable_insert(
            get_layer_env_scope(available_at_launch),
            ModificationBehavior::Delimiter,
            "PATH",
            ":",
        )
        .chainable_insert(
            get_layer_env_scope(available_at_launch),
            ModificationBehavior::Prepend,
            "PATH",
            layer_bin_dir(layer_path),
        )
        // XXX: i wanted to pass these through using SBT_OPTS instead of SBTX_OPTS but everytime i
        //      tried this the settings were not respected. i believe this is a bug in this section
        //      of the sbt-extras script:
        //      - https://github.com/dwijnand/sbt-extras/blob/master/sbt#L541-L565
        //
        //      the SBTX_OPTS variable works fine though so that's being used to ensure that sbt is pointing
        //      at all the right folders
        .chainable_insert(
            get_layer_env_scope(available_at_launch),
            ModificationBehavior::Delimiter,
            "SBTX_OPTS",
            " ",
        )
        .chainable_insert(
            get_layer_env_scope(available_at_launch),
            ModificationBehavior::Append,
            "SBTX_OPTS",
            format!(
                "-sbt-dir {} -sbt-boot {} -sbt-launch-dir {}",
                sbt_global_dir(layer_path).to_string_lossy(),
                sbt_boot_dir(layer_path).to_string_lossy(),
                sbt_launch_dir(layer_path).to_string_lossy(),
            ),
        )
}

fn get_layer_env_scope(available_at_launch: Option<bool>) -> Scope {
    if available_at_launch.unwrap_or_default() {
        Scope::All
    } else {
        Scope::Build
    }
}

fn write_sbt_extras_to_layer(layer_path: &Path) -> Result<(), SbtBuildpackError> {
    let sbt_extras_path = sbt_extras_path(layer_path);
    let contents = include_bytes!("../../assets/sbt-extras.sh");
    create_dir_all(layer_bin_dir(layer_path))
        .map_err(SbtBuildpackError::CouldNotWriteSbtExtrasScript)?;
    write(&sbt_extras_path, contents).map_err(SbtBuildpackError::CouldNotWriteSbtExtrasScript)?;
    set_permissions(&sbt_extras_path, Permissions::from_mode(0o755))
        .map_err(SbtBuildpackError::CouldNotSetExecutableBitForSbtExtrasScript)?;
    Ok(())
}

fn write_sbt_wrapper_to_layer(layer_path: &Path) -> Result<(), SbtBuildpackError> {
    let sbt_path = sbt_path(layer_path);
    let contents = include_bytes!("../../assets/sbt-wrapper.sh");
    create_dir_all(layer_bin_dir(layer_path))
        .map_err(SbtBuildpackError::CouldNotWriteSbtWrapperScript)?;
    write(&sbt_path, contents).map_err(SbtBuildpackError::CouldNotWriteSbtWrapperScript)?;
    set_permissions(&sbt_path, Permissions::from_mode(0o755))
        .map_err(SbtBuildpackError::CouldNotSetExecutableBitForSbtWrapperScript)?;
    Ok(())
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

fn layer_bin_dir(layer_path: &Path) -> PathBuf {
    layer_path.join("bin")
}

fn sbt_extras_path(layer_path: &Path) -> PathBuf {
    layer_bin_dir(layer_path).join("sbt-extras")
}

fn sbt_path(layer_path: &Path) -> PathBuf {
    layer_bin_dir(layer_path).join("sbt")
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
    use crate::layers::sbt::{
        sbt_global_plugins_dir, write_buildpack_plugin, write_sbt_extras_to_layer,
    };
    use semver::Version;
    use tempfile::tempdir;

    #[test]
    fn test_sbt_extras_is_added_to_layer() {
        let tmp = tempdir().unwrap();
        let layer_path = tmp.path();
        let sbt_extras_path = layer_path.join("bin/sbt-extras");
        write_sbt_extras_to_layer(layer_path).unwrap();
        assert!(sbt_extras_path.exists());
    }

    /*
    #[test]
    fn create_sbt_layer_env_sets_env_properly() {
        let layer_path = Path::new("./test_layer");
        let layer_env = create_sbt_layer_env(layer_path, &None, None);
        let env = layer_env.apply_to_empty(Scope::Build);
        assert_eq!(
            env.get("SBTX_OPTS").unwrap(),
            "-sbt-dir ./test_layer/global -sbt-boot ./test_layer/boot -sbt-launch-dir ./test_layer/launch"
        );
        assert_eq!(env.get("PATH").unwrap(), "./test_layer/bin");
        assert!(!env.contains_key("SBT_OPTS"));
    }

    #[test]
    fn create_sbt_layer_env_sets_env_properly_when_sbt_opts_are_present() {
        let layer_path = Path::new("./test_layer");
        let sbt_opts = vec!["-J-Xfoo".to_string()];
        let layer_env = create_sbt_layer_env(layer_path, &Some(sbt_opts), None);
        let env = layer_env.apply_to_empty(Scope::Build);
        assert_eq!(
            env.get("SBTX_OPTS").unwrap(),
            "-sbt-dir ./test_layer/global -sbt-boot ./test_layer/boot -sbt-launch-dir ./test_layer/launch"
        );
        assert_eq!(env.get("PATH").unwrap(), "./test_layer/bin");
        assert_eq!(env.get("SBT_OPTS").unwrap(), "-J-Xfoo");
    }*/

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
