use std::path::{Path, PathBuf};

/// Returns an iterator over the contents of the given directory.
///
/// This function is similar to [`std::fs::read_dir`], but collects the errors for the directory
/// entries before returning an iterator to simplify usage.
///
/// # Errors
/// - The provided path doesn't exist.
/// - The process lacks permissions to view the contents.
/// - The path points at a non-directory file.
/// - An error occurred while reading an entry of the given directory.
pub fn list_directory_contents<P>(path: P) -> std::io::Result<impl Iterator<Item = PathBuf>>
where
    P: AsRef<Path>,
{
    std::fs::read_dir(path.as_ref())
        .and_then(Iterator::collect::<std::io::Result<Vec<_>>>)
        .map(|dir_entries| dir_entries.into_iter().map(|dir_entry| dir_entry.path()))
}
