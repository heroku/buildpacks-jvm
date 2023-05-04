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
    none_on_not_found(fs::File::open(app_dir.join("system.properties")))
        .map_err(ReadSystemPropertiesError::IoError)
        .and_then(|optional_file| {
            optional_file
                .map(java_properties::read)
                .transpose()
                .map_err(ReadSystemPropertiesError::ParseError)
                .map(Option::unwrap_or_default)
        })
}

#[derive(Debug)]
pub enum ReadSystemPropertiesError {
    IoError(std::io::Error),
    ParseError(java_properties::PropertiesError),
}
