use std::borrow::Borrow;
use std::fs::Permissions;
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

#[cfg(unix)]
pub fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.as_ref()
        .metadata()
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .ok()
        .unwrap_or_default()
}

#[cfg(unix)]
#[allow(clippy::missing_errors_doc)]
pub fn set_executable<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let permissions = path.as_ref().metadata()?.permissions();
    let new_permissions = Permissions::from_mode(permissions.mode() | 0o111);

    std::fs::set_permissions(path.borrow(), new_permissions)
}
