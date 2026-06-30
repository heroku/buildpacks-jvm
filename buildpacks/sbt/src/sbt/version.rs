use buildpacks_jvm_shared::result::none_on_not_found;
use std::fs;
use std::path::Path;

/// Reads the sbt version of the app from the `sbt.version` key in the `project/build.properties` file.
///
/// That file is optional (see <https://www.scala-sbt.org/1.x/docs/sbt-by-example.html>) and if the
/// file is not present the version will be `None` instead of an error.
///
/// The version will be parsed via the `semver` crate. Even though `sbt` didn't follow semver prior
/// to sbt 1.x, all earlier versions are semver compatible so that parsing is reasonable for our
/// use in the buildpack.
pub(crate) fn read_sbt_version(
    app_dir: &Path,
) -> Result<Option<semver::Version>, ReadSbtVersionError> {
    let build_properties_path = app_dir.join("project").join("build.properties");

    none_on_not_found(fs::File::open(build_properties_path))
        .map_err(ReadSbtVersionError::CouldNotReadBuildProperties)
        .and_then(|file| {
            file.map(|file| {
                java_properties::read(file)
                    .map_err(ReadSbtVersionError::CouldNotParseBuildProperties)
                    .and_then(|properties| {
                        properties
                            .get("sbt.version")
                            .filter(|value| !value.is_empty())
                            .ok_or(ReadSbtVersionError::MissingVersionProperty)
                            .and_then(|version_string| {
                                semver::Version::parse(version_string).map_err(|error| {
                                    ReadSbtVersionError::CouldNotParseVersion(
                                        version_string.clone(),
                                        error,
                                    )
                                })
                            })
                    })
            })
            .transpose()
        })
}

#[derive(Debug)]
pub(crate) enum ReadSbtVersionError {
    CouldNotReadBuildProperties(std::io::Error),
    CouldNotParseBuildProperties(java_properties::PropertiesError),
    MissingVersionProperty,
    CouldNotParseVersion(String, semver::Error),
}

/// Checks if the given sbt version is a version that is supported by this buildpack.
///
/// sbt versions outside the `1.x` and `2.x` series aren't supported by the upstream project anymore
/// and this buildpack dropped support for those versions as well. sbt `2.0.0` is excluded because
/// of a bug that prevents global plugins from being executed, which are required by this
/// buildpack. The bug was fixed in `2.0.1`. See: <https://github.com/sbt/sbt/pull/9391>
pub(crate) fn is_supported_sbt_version(version: &semver::Version) -> bool {
    [">=1, <2", ">2.0.0, <3"]
        .into_iter()
        .map(|version_req_string| {
            semver::VersionReq::parse(version_req_string).expect("valid semver version requirement")
        })
        .any(|version_req| version_req.matches(version))
}
