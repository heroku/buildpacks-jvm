use crate::{
    util, OpenJdkBuildpack, OpenJdkBuildpackError, JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER,
    JAVA_TOOL_OPTIONS_ENV_VAR_NAME, JDK_OVERLAY_DIR_NAME,
};
use fs_extra::dir::CopyOptions;
use libcnb::additional_buildpack_binary_path;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

pub struct OpenJdkLayer {
    pub tarball_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenJdkLayerMetadata {
    jdk_overlay_applied: bool,
    source_tarball_url: String,
}

impl Layer for OpenJdkLayer {
    type Buildpack = OpenJdkBuildpack;
    type Metadata = OpenJdkLayerMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            launch: true,
            build: true,
            cache: true,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, OpenJdkBuildpackError> {
        libherokubuildpack::log::log_header("Installing OpenJDK");

        let temp_dir = tempdir().map_err(OpenJdkBuildpackError::CannotCreateOpenJdkTempDir)?;
        let path = temp_dir.path().join("openjdk.tar.gz");

        libherokubuildpack::download::download_file(&self.tarball_url, &path)
            .map_err(OpenJdkBuildpackError::OpenJdkDownloadError)?;

        std::fs::File::open(&path)
            .map_err(OpenJdkBuildpackError::CannotOpenOpenJdkTarball)
            .and_then(|mut file| {
                libherokubuildpack::tar::decompress_tarball(&mut file, layer_path)
                    .map_err(OpenJdkBuildpackError::CannotDecompressOpenJdkTarball)
            })?;

        let app_jdk_overlay_dir_path = context.app_dir.join(JDK_OVERLAY_DIR_NAME);

        let ubuntu_java_cacerts_file_path = PathBuf::from("/etc/ssl/certs/java/cacerts");

        // Depending on OpenJDK version, the path for the cacerts file can differ.
        let relative_jdk_cacerts_path = ["jre/lib/security/cacerts", "lib/security/cacerts"]
            .iter()
            .find(|path| layer_path.join(path).is_file())
            .ok_or(OpenJdkBuildpackError::MissingJdkCertificatesFile)?;

        let symlink_ubuntu_java_cacerts_file = ubuntu_java_cacerts_file_path.is_file()
            && !app_jdk_overlay_dir_path
                .join(relative_jdk_cacerts_path)
                .exists();

        if symlink_ubuntu_java_cacerts_file {
            let absolute_jdk_cacerts_path = layer_path.join(relative_jdk_cacerts_path);

            fs::rename(
                &absolute_jdk_cacerts_path,
                &absolute_jdk_cacerts_path.with_extension("old"),
            )
            .map_err(OpenJdkBuildpackError::CannotSymlinkUbuntuCertificates)?;

            // We symlink instead of copying to ensure cacerts is always the latest version,
            // even when the image is rebased.
            std::os::unix::fs::symlink(ubuntu_java_cacerts_file_path, absolute_jdk_cacerts_path)
                .map_err(OpenJdkBuildpackError::CannotSymlinkUbuntuCertificates)?;
        }

        let mut jdk_overlay_applied = false;
        if app_jdk_overlay_dir_path.is_dir() {
            jdk_overlay_applied = true;

            let jdk_overlay_contents = util::list_directory_contents(&app_jdk_overlay_dir_path)
                .map_err(OpenJdkBuildpackError::CannotListJdkOverlayContents)?;

            fs_extra::copy_items(
                &jdk_overlay_contents,
                layer_path,
                &CopyOptions {
                    overwrite: true,
                    skip_exist: false,
                    copy_inside: true,
                    ..CopyOptions::default()
                },
            )
            .map_err(OpenJdkBuildpackError::CannotCopyJdkOverlayContents)?;
        }

        LayerResultBuilder::new(OpenJdkLayerMetadata {
            source_tarball_url: self.tarball_url.clone(),
            jdk_overlay_applied,
        })
        .env(
            LayerEnv::new()
                .chainable_insert(
                    Scope::All,
                    ModificationBehavior::Override,
                    "JAVA_HOME",
                    layer_path,
                )
                .chainable_insert(
                    Scope::All,
                    ModificationBehavior::Delimiter,
                    JAVA_TOOL_OPTIONS_ENV_VAR_NAME,
                    JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER,
                )
                .chainable_insert(
                    Scope::All,
                    ModificationBehavior::Prepend,
                    JAVA_TOOL_OPTIONS_ENV_VAR_NAME,
                    "-Dfile.encoding=UTF-8",
                ),
        )
        .exec_d_program(
            "heroku_dynamic_jvm_opts",
            additional_buildpack_binary_path!("heroku_dynamic_jvm_opts"),
        )
        .build()
    }

    fn existing_layer_strategy(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, OpenJdkBuildpackError> {
        if context.app_dir.join(JDK_OVERLAY_DIR_NAME).exists()
            || layer_data.content_metadata.metadata.jdk_overlay_applied
        {
            // Since the JDK overlay will modify the OpenJDK distribution and the cached version
            // might already have an (potentially different) overlay applied, we re-crate the layer
            // in that case.
            Ok(ExistingLayerStrategy::Recreate)
        } else if self.tarball_url == layer_data.content_metadata.metadata.source_tarball_url {
            Ok(ExistingLayerStrategy::Keep)
        } else {
            Ok(ExistingLayerStrategy::Recreate)
        }
    }
}
