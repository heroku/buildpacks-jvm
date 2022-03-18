use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub enum Mode {
    UseWrapper,
    InstallVersion {
        version: String,
        warn_about_unused_maven_wrapper: bool,
        warn_about_default_version: bool,
    },
}

pub fn determine_mode<P: AsRef<Path>, S: Into<String>>(
    app_dir: P,
    default_version: S,
) -> Result<Mode, SystemPropertiesError> {
    let app_contains_maven_wrapper = app_contains_maven_wrapper(&app_dir);

    app_configured_maven_version(&app_dir).map(|app_configured_maven_version| {
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

fn app_configured_maven_version<P: AsRef<Path>>(
    app_dir: P,
) -> Result<Option<String>, SystemPropertiesError> {
    Some(app_dir.as_ref().join("system.properties"))
        .filter(|path| path.exists())
        .map_or_else(
            || Ok(None),
            |system_properties_path| {
                File::open(&system_properties_path)
                    .map_err(SystemPropertiesError::IoError)
                    .and_then(|file| {
                        java_properties::read(file)
                            .map_err(SystemPropertiesError::PropertiesError)
                            .map(|properties| properties.get("maven.version").cloned())
                    })
            },
        )
}

#[derive(Debug)]
pub enum SystemPropertiesError {
    IoError(std::io::Error),
    PropertiesError(java_properties::PropertiesError),
}
