use crate::errors::ScalaBuildpackError;
use crate::errors::ScalaBuildpackError::{
    CouldNotSetExecutableBitForSbtExtrasScript, CouldNotWriteSbtExtrasScript,
    CouldNotWriteSbtPlugin, NoBuildpackPluginAvailable, SbtInstallIoError,
    SbtInstallUnexpectedExitCode,
};
use crate::ScalaBuildpack;
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

pub struct SbtLayer {
    pub sbt_version: Version,
    pub env: Env,
}

impl Layer for SbtLayer {
    type Buildpack = ScalaBuildpack;
    type Metadata = SbtLayerMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: true,
            cache: true,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        log_info(format!("Setting up sbt {}", self.sbt_version));
        let layer_env = create_sbt_layer_env(layer_path);
        let env = layer_env.apply(Scope::Build, &self.env);
        write_sbt_extras_to_layer(layer_path)?;
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
) -> Result<ExitStatus, ScalaBuildpackError> {
    Command::new(sbt_extras_path(layer_path))
        .current_dir(app_dir)
        .args(["sbtVersion"])
        .envs(env)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(SbtInstallIoError)
        .and_then(|exit_status| {
            if exit_status.success() {
                Ok(exit_status)
            } else {
                Err(SbtInstallUnexpectedExitCode(exit_status))
            }
        })
}

fn create_sbt_layer_env(layer_path: &Path) -> LayerEnv {
    LayerEnv::new()
        .chainable_insert(
            Scope::Build,
            ModificationBehavior::Delimiter,
            "SBTX_OPTS",
            " ",
        )
        .chainable_insert(
            Scope::Build,
            ModificationBehavior::Append,
            "SBTX_OPTS",
            shell_words::join([
                "-sbt-dir",
                sbt_global_dir(layer_path).display().to_string().as_str(),
                "-sbt-boot",
                sbt_boot_dir(layer_path).display().to_string().as_str(),
                "-sbt-launch-dir",
                sbt_launch_dir(layer_path).display().to_string().as_str(),
            ]),
        )
}

fn write_sbt_extras_to_layer(layer_path: &Path) -> Result<(), ScalaBuildpackError> {
    let sbt_extras_path = sbt_extras_path(layer_path);
    let contents = include_bytes!("../../assets/sbt-extras.sh");
    create_dir_all(layer_bin_dir(layer_path)).map_err(CouldNotWriteSbtExtrasScript)?;
    write(&sbt_extras_path, contents).map_err(CouldNotWriteSbtExtrasScript)?;
    set_permissions(&sbt_extras_path, Permissions::from_mode(0o755))
        .map_err(CouldNotSetExecutableBitForSbtExtrasScript)?;
    Ok(())
}

fn write_buildpack_plugin(
    layer_path: &Path,
    sbt_version: &Version,
) -> Result<(), ScalaBuildpackError> {
    let plugin_directory = sbt_global_plugins_dir(layer_path);
    create_dir_all(&plugin_directory).map_err(CouldNotWriteSbtPlugin)?;

    let contents = get_buildpack_plugin_contents(sbt_version)?;
    write(
        plugin_directory.join("HerokuBuildpackPlugin.scala"),
        contents,
    )
    .map_err(CouldNotWriteSbtPlugin)?;

    Ok(())
}

fn get_buildpack_plugin_contents(
    sbt_version: &Version,
) -> Result<&'static [u8], ScalaBuildpackError> {
    match sbt_version {
        Version { major: 0, .. } => Ok(include_bytes!(
            "../../assets/heroku_buildpack_plugin_sbt_v0.scala"
        )),
        Version { major: 1, .. } => Ok(include_bytes!(
            "../../assets/heroku_buildpack_plugin_sbt_v1.scala"
        )),
        _ => Err(NoBuildpackPluginAvailable(sbt_version.to_string())),
    }
}

fn layer_bin_dir(layer_path: &Path) -> PathBuf {
    layer_path.join("bin")
}

fn sbt_extras_path(layer_path: &Path) -> PathBuf {
    layer_bin_dir(layer_path).join("sbt-extras")
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
mod sbt_layer_tests {
    use crate::layers::sbt::{
        create_sbt_layer_env, sbt_boot_dir, sbt_global_dir, sbt_global_plugins_dir, sbt_launch_dir,
        write_buildpack_plugin, write_sbt_extras_to_layer,
    };
    use libcnb::layer_env::Scope;
    use semver::Version;
    use std::ffi::OsString;
    use tempfile::tempdir;

    #[test]
    fn test_sbt_extras_is_added_to_layer() {
        let tmp = tempdir().unwrap();
        let layer_path = tmp.path();
        let sbt_extras_path = layer_path.join("bin/sbt-extras");
        write_sbt_extras_to_layer(layer_path).unwrap();
        assert!(sbt_extras_path.exists());
    }

    #[test]
    fn create_sbt_layer_env_sets_ivy_flag_in_sbtx_opts() {
        let layer_path = tempdir().unwrap();
        let layer_env = create_sbt_layer_env(layer_path.path());
        let env = layer_env.apply_to_empty(Scope::Build);
        assert_eq!(
            env.get("SBTX_OPTS").unwrap(),
            OsString::from(format!(
                "-sbt-dir {} -sbt-boot {} -sbt-launch-dir {}",
                sbt_global_dir(layer_path.path()).to_string_lossy(),
                sbt_boot_dir(layer_path.path()).to_string_lossy(),
                sbt_launch_dir(layer_path.path()).to_string_lossy()
            ))
        );
    }

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
pub struct SbtLayerMetadata {
    sbt_version: Version,
    layer_version: String,
    stack_id: StackId,
}

const LAYER_VERSION: &str = "v1";

impl SbtLayerMetadata {
    fn current(layer: &SbtLayer, context: &BuildContext<ScalaBuildpack>) -> Self {
        SbtLayerMetadata {
            sbt_version: layer.sbt_version.clone(),
            stack_id: context.stack_id.clone(),
            layer_version: String::from(LAYER_VERSION),
        }
    }
}
