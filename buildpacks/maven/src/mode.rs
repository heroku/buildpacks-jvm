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

#[derive(Debug)]
pub enum DetermineModeError {
    SystemPropertiesIoError(std::io::Error),
    SystemPropertiesPropertiesError(java_properties::PropertiesError),
}

pub fn determine_mode<P: AsRef<Path>, S: Into<String>>(
    app_dir: P,
    default_version: S,
) -> Result<Mode, DetermineModeError> {
    let app_contains_maven_wrapper = ["mvnw", ".mvn/wrapper/maven-wrapper.properties"]
        .iter()
        .map(|path| app_dir.as_ref().join(path))
        .all(|path| path.exists());

    let app_configured_maven_version = Some(app_dir.as_ref().join("system.properties"))
        .filter(|path| path.exists())
        .map_or_else(
            || Ok(None),
            |system_properties_path| {
                File::open(&system_properties_path)
                    .map_err(DetermineModeError::SystemPropertiesIoError)
                    .and_then(|file| {
                        java_properties::read(file)
                            .map_err(DetermineModeError::SystemPropertiesPropertiesError)
                            .map(|properties| properties.get("maven.version").cloned())
                    })
            },
        );

    app_configured_maven_version.map(|app_configured_maven_version| {
        match app_configured_maven_version {
            None => {
                if app_contains_maven_wrapper {
                    Mode::UseWrapper
                } else {
                    Mode::InstallVersion {
                        version: default_version.into(),
                        warn_about_default_version: true,
                        warn_about_unused_maven_wrapper: false,
                    }
                }
            }
            Some(version) => {
                if app_contains_maven_wrapper {
                    Mode::InstallVersion {
                        version,
                        warn_about_unused_maven_wrapper: true,
                        warn_about_default_version: false,
                    }
                } else {
                    Mode::InstallVersion {
                        version,
                        warn_about_unused_maven_wrapper: false,
                        warn_about_default_version: false,
                    }
                }
            }
        }
    })
}
