use buildpacks_jvm_shared::default_on_not_found;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub(crate) fn read_system_properties(
    app_dir: &Path,
) -> Result<HashMap<String, String>, ReadSystemPropertiesError> {
    default_on_not_found(fs::read(app_dir.join("system.properties")))
        .map_err(ReadSystemPropertiesError::IoError)
        .and_then(|file_contents| {
            java_properties::read(&file_contents[..]).map_err(ReadSystemPropertiesError::ParseError)
        })
}

#[derive(Debug)]
pub(crate) enum ReadSystemPropertiesError {
    IoError(std::io::Error),
    ParseError(java_properties::PropertiesError),
}
