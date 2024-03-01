use crate::constants::{
    JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER, JAVA_TOOL_OPTIONS_ENV_VAR_NAME, JDK_OVERLAY_DIR_NAME,
};
use crate::{util, OpenJdkBuildpack, OpenJdkBuildpackError};
use fs_extra::dir::CopyOptions;
use libcnb::additional_buildpack_binary_path;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::{
    InspectExistingAction, InvalidMetadataAction, LayerDefinition, LayerDefinitionResult,
};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct OpenJdkLayerMetadata {
    jdk_overlay_applied: bool,
    source_tarball_url: String,
}

pub(crate) fn handle(
    tarball_url: String,
    context: &BuildContext<OpenJdkBuildpack>,
) -> libcnb::Result<(), OpenJdkBuildpackError> {
    let layer = context.execute_layer_definition(
        layer_name!("openjdk"),
        LayerDefinition {
            build: false,
            launch: true,
            cache: true,
            invalid_metadata: &|_| InvalidMetadataAction::DeleteLayer,
            inspect_existing: &|metadata, _| {
                if context.app_dir.join(JDK_OVERLAY_DIR_NAME).exists()
                    || metadata.jdk_overlay_applied
                {
                    // Since the JDK overlay will modify the OpenJDK distribution and the cached version
                    // might already have an (potentially different) overlay applied, we re-crate the layer
                    // in that case.
                    InspectExistingAction::Delete
                } else if tarball_url == metadata.source_tarball_url {
                    InspectExistingAction::Keep
                } else {
                    InspectExistingAction::Delete
                }
            },
        },
    )?;

    if let LayerDefinitionResult::Empty { layer_data, .. } = layer {
        libherokubuildpack::log::log_header("Installing OpenJDK");

        let temp_dir = tempdir().map_err(OpenJdkBuildpackError::CannotCreateOpenJdkTempDir)?;
        let path = temp_dir.path().join("openjdk.tar.gz");

        libherokubuildpack::download::download_file(&tarball_url, &path)
            .map_err(OpenJdkBuildpackError::OpenJdkDownloadError)?;

        std::fs::File::open(&path)
            .map_err(OpenJdkBuildpackError::CannotOpenOpenJdkTarball)
            .and_then(|mut file| {
                libherokubuildpack::tar::decompress_tarball(&mut file, &layer_data.path)
                    .map_err(OpenJdkBuildpackError::CannotDecompressOpenJdkTarball)
            })?;

        let app_jdk_overlay_dir_path = context.app_dir.join(JDK_OVERLAY_DIR_NAME);

        let ubuntu_java_cacerts_file_path = PathBuf::from("/etc/ssl/certs/java/cacerts");

        // Depending on OpenJDK version, the path for the cacerts file can differ.
        let relative_jdk_cacerts_path = ["jre/lib/security/cacerts", "lib/security/cacerts"]
            .iter()
            .find(|path| layer_data.path.join(path).is_file())
            .ok_or(OpenJdkBuildpackError::MissingJdkCertificatesFile)?;

        let symlink_ubuntu_java_cacerts_file = ubuntu_java_cacerts_file_path.is_file()
            && !app_jdk_overlay_dir_path
                .join(relative_jdk_cacerts_path)
                .exists();

        if symlink_ubuntu_java_cacerts_file {
            let absolute_jdk_cacerts_path = layer_data.path.join(relative_jdk_cacerts_path);

            fs::rename(
                &absolute_jdk_cacerts_path,
                absolute_jdk_cacerts_path.with_extension("old"),
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
                &layer_data.path,
                &CopyOptions {
                    overwrite: true,
                    skip_exist: false,
                    copy_inside: true,
                    ..CopyOptions::default()
                },
            )
            .map_err(OpenJdkBuildpackError::CannotCopyJdkOverlayContents)?;
        }

        libcnb::layer::replace_metadata(
            OpenJdkLayerMetadata {
                jdk_overlay_applied,
                source_tarball_url: tarball_url.clone(),
            },
            &layer_data.path,
        )?;

        libcnb::layer::replace_env(
            LayerEnv::new()
                .chainable_insert(
                    Scope::All,
                    ModificationBehavior::Override,
                    "JAVA_HOME",
                    layer_data.path,
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
            &layer_data.path,
        )?;

        libcnb::layer::replace_execd_programs(
            &[(
                "heroku_dynamic_jvm_opts",
                &additional_buildpack_binary_path!("heroku_dynamic_jvm_opts"),
            )],
            &layer_data.path,
        )?;
    }

    Ok(())
}
