use buildpacks_jvm_shared::system_properties::{ReadSystemPropertiesError, read_system_properties};
use std::path::Path;

#[derive(Debug)]
pub(crate) enum Mode {
    UseWrapper,
    InstallVersion {
        version: String,
        warn_about_unused_maven_wrapper: bool,
        warn_about_default_version: bool,
    },
}

pub(crate) fn determine_mode<P: AsRef<Path>, S: Into<String>>(
    app_dir: P,
    default_version: S,
) -> Result<Mode, ReadSystemPropertiesError> {
    let app_contains_maven_wrapper = app_contains_maven_wrapper(&app_dir);

    read_system_properties(app_dir.as_ref())
        .map(|properties| properties.get("maven.version").cloned())
        .map(|app_configured_maven_version| {
            if app_contains_maven_wrapper && app_configured_maven_version.is_none() {
                Mode::UseWrapper
            } else {
                Mode::InstallVersion {
                    version: app_configured_maven_version
                        .clone()
                        .unwrap_or_else(|| default_version.into()),
                    warn_about_default_version: app_configured_maven_version.is_none(),
                    warn_about_unused_maven_wrapper: app_contains_maven_wrapper,
                }
            }
        })
}

fn app_contains_maven_wrapper<P: AsRef<Path>>(app_dir: P) -> bool {
    ["mvnw", ".mvn/wrapper/maven-wrapper.properties"]
        .iter()
        .map(|path| app_dir.as_ref().join(path))
        .all(|path| path.exists())
}
