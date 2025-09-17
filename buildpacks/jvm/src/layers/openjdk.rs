use crate::openjdk_artifact::OpenJdkArtifactMetadata;
use crate::openjdk_version::OpenJdkVersion;
use crate::util::digest;
use crate::{
    JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER, JAVA_TOOL_OPTIONS_ENV_VAR_NAME, JDK_OVERLAY_DIR_NAME,
    OpenJdkBuildpack, OpenJdkBuildpackError, util,
};
use buildpacks_jvm_shared::output;
use buildpacks_jvm_shared::output::{BuildpackOutputText, BuildpackOutputTextSection};
use fs_extra::dir::CopyOptions;
use libcnb::additional_buildpack_binary_path;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::{
    CachedLayerDefinition, EmptyLayerCause, InvalidMetadataAction, LayerState, RestoredLayerAction,
};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libherokubuildpack::inventory::artifact::Artifact;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use std::path::PathBuf;
use tempfile::tempdir;

#[allow(clippy::too_many_lines)]
pub(crate) fn handle_openjdk_layer(
    context: &BuildContext<OpenJdkBuildpack>,
    artifact: &Artifact<OpenJdkVersion, Sha256, OpenJdkArtifactMetadata>,
) -> libcnb::Result<(), OpenJdkBuildpackError> {
    output::print_section("OpenJDK Installation");

    let layer_ref = context.cached_layer(
        layer_name!("openjdk"),
        CachedLayerDefinition {
            build: true,
            launch: true,
            invalid_metadata_action: &|_| InvalidMetadataAction::DeleteLayer,
            restored_layer_action: &|metadata: &OpenJdkLayerMetadata, _| {
                if context.app_dir.join(JDK_OVERLAY_DIR_NAME).exists()
                    || metadata.jdk_overlay_applied
                {
                    // Since the JDK overlay will modify the OpenJDK distribution and the cached version
                    // might already have a (potentially different) overlay applied, we re-crate the layer
                    // in that case.
                    (
                        RestoredLayerAction::DeleteLayer,
                        OpenJdkLayerCause::OverlayUsed,
                    )
                } else if artifact.url != metadata.source_tarball_url {
                    (
                        RestoredLayerAction::DeleteLayer,
                        OpenJdkLayerCause::VersionChanged,
                    )
                } else {
                    (
                        RestoredLayerAction::KeepLayer,
                        OpenJdkLayerCause::RestoredLayerValid,
                    )
                }
            },
        },
    )?;

    match layer_ref.state {
        LayerState::Restored { .. } => {
            output::print_subsection("Using cached OpenJDK installation from previous build");
        }
        LayerState::Empty { ref cause } => {
            match cause {
                EmptyLayerCause::InvalidMetadataAction { .. } => {
                    output::print_subsection("Clearing OpenJDK cache (invalid metadata)");
                }
                EmptyLayerCause::RestoredLayerAction {
                    cause: OpenJdkLayerCause::OverlayUsed,
                } => output::print_subsection("Clearing OpenJDK cache (JDK overlay used)"),
                EmptyLayerCause::RestoredLayerAction {
                    cause: OpenJdkLayerCause::VersionChanged,
                } => output::print_subsection("Clearing OpenJDK cache (version changed)"),
                _ => {}
            }

            output::track_subsection_timing(|| {
                output::print_subsection("Downloading and unpacking OpenJDK distribution");

                let temp_dir =
                    tempdir().map_err(OpenJdkBuildpackError::CannotCreateOpenJdkTempDir)?;
                let path = temp_dir.path().join("openjdk.tar.gz");

                libherokubuildpack::download::download_file(&artifact.url, &path)
                    .map_err(OpenJdkBuildpackError::OpenJdkDownloadError)?;

                std::fs::File::open(&path)
                    .map_err(OpenJdkBuildpackError::CannotReadOpenJdkTarball)
                    .and_then(|file| {
                        digest::<Sha256>(file)
                            .map_err(OpenJdkBuildpackError::CannotReadOpenJdkTarball)
                    })
                    .and_then(|downloaded_file_digest| {
                        if downloaded_file_digest.as_slice() == artifact.checksum.value {
                            Ok(())
                        } else {
                            Err(OpenJdkBuildpackError::OpenJdkTarballChecksumError {
                                expected: artifact.checksum.value.clone(),
                                actual: downloaded_file_digest.to_vec(),
                            })
                        }
                    })?;

                std::fs::File::open(&path)
                    .map_err(OpenJdkBuildpackError::CannotReadOpenJdkTarball)
                    .and_then(|mut file| {
                        libherokubuildpack::tar::decompress_tarball(&mut file, layer_ref.path())
                            .map_err(OpenJdkBuildpackError::CannotDecompressOpenJdkTarball)
                    })
            })?;

            output::print_section("Applying JDK overlay");
            let app_jdk_overlay_dir_path = context.app_dir.join(JDK_OVERLAY_DIR_NAME);

            let mut jdk_overlay_applied = false;
            if app_jdk_overlay_dir_path.is_dir() {
                output::print_subsection(BuildpackOutputText::new(vec![
                    BuildpackOutputTextSection::regular("Copying files from "),
                    BuildpackOutputTextSection::value(JDK_OVERLAY_DIR_NAME),
                    BuildpackOutputTextSection::regular(" to OpenJDK directory"),
                ]));

                jdk_overlay_applied = true;

                output::track_subsection_timing(|| {
                    let jdk_overlay_contents =
                        util::list_directory_contents(&app_jdk_overlay_dir_path)
                            .map_err(OpenJdkBuildpackError::CannotListJdkOverlayContents)?;

                    fs_extra::copy_items(
                        &jdk_overlay_contents,
                        layer_ref.path(),
                        &CopyOptions {
                            overwrite: true,
                            skip_exist: false,
                            copy_inside: true,
                            ..CopyOptions::default()
                        },
                    )
                    .map_err(OpenJdkBuildpackError::CannotCopyJdkOverlayContents)
                })?;
            } else {
                output::print_subsection(BuildpackOutputText::new(vec![
                    BuildpackOutputTextSection::regular("Skipping (directory "),
                    BuildpackOutputTextSection::value(JDK_OVERLAY_DIR_NAME),
                    BuildpackOutputTextSection::regular(" not present)"),
                ]));
            }

            output::print_section("Linking base image certificates as OpenJDK keystore");

            // Depending on OpenJDK version, the path for the cacerts file can differ.
            let relative_jdk_cacerts_path = ["jre/lib/security/cacerts", "lib/security/cacerts"]
                .iter()
                .find(|path| layer_ref.path().join(path).is_file())
                .ok_or(OpenJdkBuildpackError::MissingJdkCertificatesFile)?;

            let overlay_has_cacerts = app_jdk_overlay_dir_path
                .join(relative_jdk_cacerts_path)
                .exists();

            let ubuntu_java_cacerts_file_path = PathBuf::from("/etc/ssl/certs/java/cacerts");

            if overlay_has_cacerts {
                output::print_subsection(BuildpackOutputText::new(vec![
                    BuildpackOutputTextSection::regular("Skipping (overlay at "),
                    BuildpackOutputTextSection::value(JDK_OVERLAY_DIR_NAME),
                    BuildpackOutputTextSection::regular(" contains "),
                    BuildpackOutputTextSection::value(*relative_jdk_cacerts_path),
                    BuildpackOutputTextSection::regular("file)"),
                ]));
            } else if ubuntu_java_cacerts_file_path.is_file() {
                let absolute_jdk_cacerts_path = layer_ref.path().join(relative_jdk_cacerts_path);

                std::fs::rename(
                    &absolute_jdk_cacerts_path,
                    absolute_jdk_cacerts_path.with_extension("old"),
                )
                .map_err(OpenJdkBuildpackError::CannotSymlinkUbuntuCertificates)?;

                // We symlink instead of copying to ensure cacerts is always the latest version,
                // even when the image is rebased.
                std::os::unix::fs::symlink(
                    ubuntu_java_cacerts_file_path,
                    absolute_jdk_cacerts_path,
                )
                .map_err(OpenJdkBuildpackError::CannotSymlinkUbuntuCertificates)?;

                output::print_subsection("Done");
            } else {
                output::print_subsection(BuildpackOutputText::new(vec![
                    BuildpackOutputTextSection::regular("Skipping ("),
                    BuildpackOutputTextSection::value(
                        ubuntu_java_cacerts_file_path.to_string_lossy(),
                    ),
                    BuildpackOutputTextSection::regular(" does not exist)"),
                ]));
            }

            layer_ref.write_metadata(OpenJdkLayerMetadata {
                source_tarball_url: artifact.url.clone(),
                jdk_overlay_applied,
            })?;

            layer_ref.write_env(
                LayerEnv::new()
                    .chainable_insert(
                        Scope::All,
                        ModificationBehavior::Override,
                        "JAVA_HOME",
                        layer_ref.path(),
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
            )?;

            layer_ref.write_exec_d_programs([(
                "heroku_dynamic_jvm_opts",
                additional_buildpack_binary_path!("heroku_dynamic_jvm_opts"),
            )])?;
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct OpenJdkLayerMetadata {
    jdk_overlay_applied: bool,
    source_tarball_url: String,
}

pub(crate) enum OpenJdkLayerCause {
    OverlayUsed,
    VersionChanged,
    RestoredLayerValid,
}
