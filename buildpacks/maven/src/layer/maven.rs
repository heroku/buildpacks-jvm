use crate::{util, MavenBuildpack, MavenBuildpackError, Tarball};
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

pub(crate) struct MavenLayer {
    pub(crate) tarball: Tarball,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MavenLayerMetadata {
    tarball: Tarball,
}

impl Layer for MavenLayer {
    type Buildpack = MavenBuildpack;
    type Metadata = MavenLayerMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            launch: true,
            build: true,
            cache: true,
        }
    }

    fn create(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_file_path = temp_dir.path().join("maven.tar.gz");

        libherokubuildpack::download::download_file(&self.tarball.url, &temp_file_path)
            .map_err(MavenBuildpackError::MavenTarballDownloadError)?;

        libherokubuildpack::digest::sha256(&temp_file_path)
            .map_err(MavenBuildpackError::MavenTarballSha256IoError)
            .and_then(|downloaded_tarball_sha256| {
                if downloaded_tarball_sha256 == self.tarball.sha256 {
                    Ok(())
                } else {
                    Err(MavenBuildpackError::MavenTarballSha256Mismatch {
                        expected_sha256: self.tarball.sha256.clone(),
                        actual_sha256: downloaded_tarball_sha256,
                    })
                }
            })?;

        libherokubuildpack::tar::decompress_tarball(
            &mut File::open(&temp_file_path).unwrap(),
            layer_path,
        )
        .map_err(MavenBuildpackError::MavenTarballDecompressError)?;

        // The actual Maven installation is located in the .maven subdirectory of the tarball. We
        // need to move its contents to the layer itself before it can be used:
        util::move_directory_contents(layer_path.join(".maven"), layer_path)
            .map_err(MavenBuildpackError::MavenTarballNormalizationError)?;

        std::fs::remove_dir_all(layer_path.join(".maven"))
            .map_err(MavenBuildpackError::MavenTarballNormalizationError)?;

        // Heroku's Maven tarballs historically also contained a .m2 directory with pre-populated
        // dependencies for faster builds. For all recent Maven versions, this directory is
        // empty and unused.
        std::fs::remove_dir_all(layer_path.join(".m2"))
            .map_err(MavenBuildpackError::MavenTarballNormalizationError)?;

        // Even though M2_HOME is no longer supported by Maven versions >= 3.5.0, other tooling such
        // as Maven invoker might still depend on it. References:
        // - https://maven.apache.org/docs/3.5.0/release-notes.html#overview-about-the-changes
        // - https://maven.apache.org/shared/maven-invoker/usage.html
        let layer_env = LayerEnv::new().chainable_insert(
            Scope::Build,
            ModificationBehavior::Override,
            "M2_HOME",
            layer_path,
        );

        LayerResultBuilder::new(MavenLayerMetadata {
            tarball: self.tarball.clone(),
        })
        .env(layer_env)
        .build()
    }

    fn existing_layer_strategy(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        let strategy = if layer_data.content_metadata.metadata.tarball == self.tarball {
            ExistingLayerStrategy::Keep
        } else {
            ExistingLayerStrategy::Recreate
        };

        Ok(strategy)
    }
}
