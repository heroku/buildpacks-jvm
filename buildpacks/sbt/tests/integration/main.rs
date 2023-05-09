//! Bundle all integration tests into one binary to:
//! - Reduce compile times
//! - Reduce required disk space
//! - Increase parallelism
//!
//! See: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Implications

use libcnb_test::BuildpackReference;

mod caching;
mod smoke;
mod ux;

pub(crate) fn default_buildpacks() -> Vec<BuildpackReference> {
    vec![
        BuildpackReference::Other(String::from("heroku/jvm")),
        BuildpackReference::Crate,
        BuildpackReference::Other(String::from("heroku/procfile")),
    ]
}
