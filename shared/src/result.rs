/// Removes [`std::io::Error`] values from a [`Result`] that have the
/// [`std::io::ErrorKind::NotFound`] error kind by replacing them with the default value for `T`.
pub fn default_on_not_found<T: Default>(
    result: Result<T, std::io::Error>,
) -> Result<T, std::io::Error> {
    none_on_not_found(result).map(Option::unwrap_or_default)
}

/// Removes [`std::io::Error`] values from a [`Result`] that have the
/// [`std::io::ErrorKind::NotFound`] error kind by replacing the `Err(std::io::Error)` with Ok(None).
pub fn none_on_not_found<T>(
    result: Result<T, std::io::Error>,
) -> Result<Option<T>, std::io::Error> {
    match result {
        Err(io_error) if io_error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        other => other.map(Some),
    }
}
