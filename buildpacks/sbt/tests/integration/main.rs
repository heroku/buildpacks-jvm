//! Bundle all integration tests into one binary to:
//! - Reduce compile times
//! - Reduce required disk space
//! - Increase parallelism
//!
//! See: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Implications

use libcnb_test::BuildpackReference;

mod caching;
mod sbt_at_launch;
mod smoke;
mod ux;

pub(crate) fn default_buildpacks() -> Vec<BuildpackReference> {
    vec![
        BuildpackReference::Other(String::from("heroku/jvm")),
        BuildpackReference::Crate,
        // Using an explicit version from Docker Hub to prevent failures when there
        // are multiple Procfile buildpack versions in the builder image.
        BuildpackReference::Other(String::from("docker://docker.io/heroku/procfile-cnb:2.0.1")),
    ]
}
