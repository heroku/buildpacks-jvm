use std::{io, path::Path};

/// Detect if path is a maven app
///
/// # Errors
///
/// - The provided path doesn't exist.
/// - The process lacks permissions to view the contents.
/// - The path points at a non-directory file.
/// - An error occurred while reading an entry of the given directory.
pub fn detect(root_dir: &Path) -> io::Result<bool> {
    ["xml", "atom", "clj", "groovy", "rb", "scala", "yaml", "yml"]
        .iter()
        .map(|extension| root_dir.join(format!("pom.{extension}")))
        .map(|path| path.try_exists())
        .collect::<io::Result<Vec<bool>>>()
        .map(|file_exists_vec| file_exists_vec.contains(&true))
}
