use semver::{Version, VersionReq};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub(crate) enum ReadSbtVersionError {
    CouldNotReadBuildProperties(std::io::Error),
    CouldNotParseBuildProperties(java_properties::PropertiesError),
    MissingVersionProperty,
    CouldNotParseVersion(String, semver::Error),
}

pub(crate) fn read_sbt_version(app_dir: &Path) -> Result<Version, ReadSbtVersionError> {
    fs::File::open(app_dir.join("project").join("build.properties"))
        .map_err(ReadSbtVersionError::CouldNotReadBuildProperties)
        .and_then(|file| {
            java_properties::read(file).map_err(ReadSbtVersionError::CouldNotParseBuildProperties)
        })
        .and_then(|properties| {
            properties
                .get("sbt.version")
                .filter(|value| !value.is_empty())
                .ok_or(ReadSbtVersionError::MissingVersionProperty)
                .cloned()
        })
        .and_then(|version_string| {
            // While sbt didn't officially adopt semver until the 1.x version, all the published
            // versions before 1.x followed semver coincidentally.
            Version::parse(&version_string)
                .map_err(|error| ReadSbtVersionError::CouldNotParseVersion(version_string, error))
        })
}

pub(crate) fn is_supported_sbt_version(version: &Version) -> bool {
    // sbt versions outside of the 1.x series aren't supported by the upstream project anymore.
    // However, we supported 0.11.x through 0.13.x before and can continue supporting them for now.
    [">=0.11, <=0.13", ">=1, <2"]
        .into_iter()
        .map(|version_req_string| {
            VersionReq::parse(version_req_string).expect("valid semver version requirement")
        })
        .any(|version_req| version_req.matches(version))
}
