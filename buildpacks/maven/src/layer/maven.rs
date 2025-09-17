use crate::util::extract_tarball;
use crate::{MavenBuildpack, MavenBuildpackError, Tarball};
use libcnb::Env;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::{
    CachedLayerDefinition, InvalidMetadataAction, LayerState, RestoredLayerAction,
};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use serde::{Deserialize, Serialize};
use std::fs::File;

pub(crate) fn handle_maven_layer(
    context: &BuildContext<MavenBuildpack>,
    tarball: &Tarball,
    env: &mut Env,
) -> libcnb::Result<(), MavenBuildpackError> {
    let layer_ref = context.cached_layer(
        layer_name!("maven"),
        CachedLayerDefinition {
            build: true,
            launch: true,
            invalid_metadata_action: &|_| InvalidMetadataAction::DeleteLayer,
            restored_layer_action: &|metadata: &MavenLayerMetadata, _| {
                if &metadata.tarball == tarball {
                    RestoredLayerAction::KeepLayer
                } else {
                    RestoredLayerAction::DeleteLayer
                }
            },
        },
    )?;

    if let LayerState::Empty { .. } = layer_ref.state {
        let temp_dir = tempfile::tempdir()
            .map_err(MavenBuildpackError::MavenTarballCreateTemporaryDirectoryError)?;

        let temp_file_path = temp_dir.path().join("maven.tar.gz");

        libherokubuildpack::download::download_file(&tarball.url, &temp_file_path)
            .map_err(MavenBuildpackError::MavenTarballDownloadError)?;

        libherokubuildpack::digest::sha256(&temp_file_path)
            .map_err(MavenBuildpackError::MavenTarballSha256IoError)
            .and_then(|downloaded_tarball_sha256| {
                if downloaded_tarball_sha256 == tarball.sha256 {
                    Ok(())
                } else {
                    Err(MavenBuildpackError::MavenTarballSha256Mismatch {
                        expected_sha256: tarball.sha256.clone(),
                        actual_sha256: downloaded_tarball_sha256,
                    })
                }
            })?;

        File::open(&temp_file_path)
            .and_then(|mut file| extract_tarball(&mut file, &layer_ref.path(), 1))
            .map_err(MavenBuildpackError::MavenTarballDecompressError)?;

        // Even though M2_HOME is no longer supported by Maven versions >= 3.5.0, other tooling such
        // as Maven invoker might still depend on it. References:
        // - https://maven.apache.org/docs/3.5.0/release-notes.html#overview-about-the-changes
        // - https://maven.apache.org/shared/maven-invoker/usage.html
        let layer_env = LayerEnv::new().chainable_insert(
            Scope::Build,
            ModificationBehavior::Override,
            "M2_HOME",
            layer_ref.path(),
        );

        layer_ref.write_env(layer_env)?;
        layer_ref.write_metadata(MavenLayerMetadata {
            tarball: tarball.clone(),
        })?;
    }

    *env = layer_ref.read_env()?.apply(Scope::Build, env);

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MavenLayerMetadata {
    tarball: Tarball,
}
