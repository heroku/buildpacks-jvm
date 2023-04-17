// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

use indoc::formatdoc;
use libherokubuildpack::log::log_error;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

pub fn log_please_try_again_error<H: AsRef<str>, M: AsRef<str>, E: Debug>(
    header: H,
    message: M,
    error: E,
) {
    log_error(
        header,
        formatdoc! {"
            {message}

            Please try again. If this error persists, please contact us:
            https://help.heroku.com/

            Details: {error:?}
        ", message = message.as_ref(), error = error },
    );
}

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

/// Removes [`std::io::Error`] values from a [`Result`] that have the
/// [`std::io::ErrorKind::NotFound`] error kind by replacing them with the default value for `T`.
#[allow(clippy::missing_errors_doc)]
pub fn default_on_not_found<T: Default>(
    result: Result<T, std::io::Error>,
) -> Result<T, std::io::Error> {
    match result {
        Err(io_error) if is_not_found_error_kind(&io_error) => Ok(T::default()),
        other => other,
    }
}

/// Checks if the error kind of the given [`std::io::Error`]  is [`std::io::ErrorKind::NotFound`].
#[must_use]
pub fn is_not_found_error_kind(error: &std::io::Error) -> bool {
    matches!(error.kind(), std::io::ErrorKind::NotFound)
}
