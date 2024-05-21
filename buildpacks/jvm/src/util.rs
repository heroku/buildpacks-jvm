use libcnb::Env;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) enum ValidateSha256Error {
    CouldNotObtainSha256(std::io::Error),
    InvalidChecksum { actual: String, expected: String },
}

pub(crate) fn validate_sha256<P: AsRef<Path>, S: Into<String>>(
    path: P,
    expected_sha256: S,
) -> Result<(), ValidateSha256Error> {
    let expected_sha256 = expected_sha256.into();

    libherokubuildpack::digest::sha256(path.as_ref())
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

pub(crate) fn list_directory_contents<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<PathBuf>> {
    std::fs::read_dir(path.as_ref())
        .and_then(Iterator::collect::<std::io::Result<Vec<DirEntry>>>)
        .map(|dir_entries| dir_entries.iter().map(DirEntry::path).collect())
}

pub(crate) fn boolean_buildpack_config_env_var(env: &Env, key: impl AsRef<OsStr>) -> bool {
    env.get(key.as_ref())
        .is_some_and(|value| value != "0" && value != "false" && value != "no")
}

pub(crate) fn zip_longest<A, B>(a: A, b: B) -> ZipLongest<A, B>
where
    A: Iterator,
    B: Iterator,
{
    ZipLongest { a, b }
}

pub(crate) struct ZipLongest<A, B> {
    a: A,
    b: B,
}

impl<A, B> Iterator for ZipLongest<A, B>
where
    A: Iterator,
    B: Iterator,
{
    type Item = (Option<A::Item>, Option<B::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let a_item = self.a.next();
        let b_item = self.b.next();

        if a_item.is_none() && b_item.is_none() {
            None
        } else {
            Some((a_item, b_item))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::boolean_buildpack_config_env_var;
    use libcnb::Env;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test() {
        let mut env = Env::new();
        env.insert("FOO", "1");
        env.insert("BAR", "0");
        env.insert("BAZ", "false");
        env.insert("BLAH", "true");
        env.insert("BLUBB", "yes");
        env.insert("BLIPP", "no");

        assert_eq!(boolean_buildpack_config_env_var(&env, "FOO"), true);
        assert_eq!(boolean_buildpack_config_env_var(&env, "BAR"), false);
        assert_eq!(boolean_buildpack_config_env_var(&env, "BAZ"), false);
        assert_eq!(boolean_buildpack_config_env_var(&env, "BLAH"), true);
        assert_eq!(boolean_buildpack_config_env_var(&env, "BLUBB"), true);
        assert_eq!(boolean_buildpack_config_env_var(&env, "BLIPP"), false);
    }
}
