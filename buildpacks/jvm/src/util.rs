use std::path::Path;

#[derive(Debug)]
pub enum ValidateSha256Error {
    CouldNotObtainSha256(std::io::Error),
    InvalidChecksum { actual: String, expected: String },
}

pub(crate) fn validate_sha256<P: AsRef<Path>, S: Into<String>>(
    path: P,
    expected_sha256: S,
) -> Result<(), ValidateSha256Error> {
    let expected_sha256 = expected_sha256.into();

    libherokubuildpack::sha256(path.as_ref())
        .map_err(ValidateSha256Error::CouldNotObtainSha256)
        .and_then(|actual_sha256| {
            if expected_sha256 == actual_sha256 {
                Ok(())
            } else {
                Err(ValidateSha256Error::InvalidChecksum {
                    actual: actual_sha256,
                    expected: expected_sha256,
                })
            }
        })
}
