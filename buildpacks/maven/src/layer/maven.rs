use crate::util::extract_tarball;
use crate::{MavenBuildpack, MavenBuildpackError, MavenVersion};
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use url::Url;

pub(crate) struct MavenLayer {
    pub(crate) apache_maven_mirror: Url,
    pub(crate) version: MavenVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MavenLayerMetadata {
    version: MavenVersion,
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

        let maven_url = self.apache_maven_mirror.join(&self.version.path).unwrap();

        libherokubuildpack::download::download_file(maven_url, &temp_file_path)
            .map_err(MavenBuildpackError::MavenTarballDownloadError)?;

        libherokubuildpack::digest::sha256(&temp_file_path)
            .map_err(MavenBuildpackError::MavenTarballSha256IoError)
            .and_then(|downloaded_tarball_sha256| {
                if downloaded_tarball_sha256 == self.version.sha256 {
                    Ok(())
                } else {
                    Err(MavenBuildpackError::MavenTarballSha256Mismatch {
                        expected_sha256: self.version.sha256.clone(),
                        actual_sha256: downloaded_tarball_sha256,
                    })
                }
            })?;

        extract_tarball(&mut File::open(&temp_file_path).unwrap(), layer_path, 1)
            .map_err(MavenBuildpackError::MavenTarballDecompressError)?;

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
            version: self.version.clone(),
        })
        .env(layer_env)
        .build()
    }

    fn existing_layer_strategy(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        let strategy = if layer_data.content_metadata.metadata.version == self.version {
            ExistingLayerStrategy::Keep
        } else {
            ExistingLayerStrategy::Recreate
        };

        Ok(strategy)
    }
}
