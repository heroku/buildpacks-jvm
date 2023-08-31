use crate::result::none_on_not_found;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Reads and parses all properties from the `system.properties` file in the app's directory.
///
/// A missing `system.properties` file is not considered an error. The resulting `HashMap`
/// will be empty instead.
#[allow(clippy::missing_errors_doc)]
pub fn read_system_properties(
    app_dir: &Path,
) -> Result<HashMap<String, String>, ReadSystemPropertiesError> {
    none_on_not_found(fs::File::open(app_dir.join(SYSTEM_PROPERTIES_FILE_NAME)))
        .map_err(ReadSystemPropertiesError::IoError)
        .and_then(|optional_file| {
            optional_file
                .map(java_properties::read)
                .transpose()
                .map_err(ReadSystemPropertiesError::ParseError)
                .map(Option::unwrap_or_default)
        })
}

/// Writes all given properties to the `system.properties` file in the app's directory.
// Implicit hasher is allowed since the properties crate only works with the default one.
#[allow(clippy::missing_errors_doc, clippy::implicit_hasher)]
pub fn write_system_properties(
    app_dir: &Path,
    properties: &HashMap<String, String>,
) -> Result<(), WriteSystemPropertiesError> {
    fs::File::create(app_dir.join(SYSTEM_PROPERTIES_FILE_NAME))
        .map_err(WriteSystemPropertiesError::IoError)
        .and_then(|file| {
            java_properties::write(file, properties)
                .map_err(WriteSystemPropertiesError::SerializationError)
        })
}

#[derive(Debug)]
pub enum ReadSystemPropertiesError {
    IoError(std::io::Error),
    ParseError(java_properties::PropertiesError),
}

#[derive(Debug)]
pub enum WriteSystemPropertiesError {
    IoError(std::io::Error),
    SerializationError(java_properties::PropertiesError),
}

const SYSTEM_PROPERTIES_FILE_NAME: &str = "system.properties";
