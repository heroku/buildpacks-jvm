use crate::data::Runtime;
use serde::Deserialize;
use std::convert::TryFrom;
use toml::value::Table;

#[derive(Deserialize)]
pub struct Metadata {
    pub runtime: Runtime,
    pub release: Release,
}

impl TryFrom<&Table> for Metadata {
    type Error = anyhow::Error;

    fn try_from(value: &Table) -> Result<Self, Self::Error> {
        Ok(toml::from_str(&toml::to_string(&value)?)?)
    }
}

#[derive(Deserialize)]
pub struct Release {
    pub docker: Docker,
}

#[derive(Deserialize)]
pub struct Docker {
    pub repository: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn metadata_try_from_parses_vendored_buildpack_toml() -> anyhow::Result<()> {
        let buildpack_toml: libcnb::data::buildpack::BuildpackToml = toml::from_str(
            &fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("buildpack.toml"))?,
        )?;

        assert!(Metadata::try_from(&buildpack_toml.metadata).is_ok());

        let metadata = Metadata::try_from(&buildpack_toml.metadata)?;
        println!("{}", metadata.release.docker.repository);

        Ok(())
    }
}
