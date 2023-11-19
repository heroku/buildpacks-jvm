//! Bundle all integration tests into one binary to:
//! - Reduce compile times
//! - Reduce required disk space
//! - Increase parallelism
//!
//! See: <https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Implications>

// Required due to: https://github.com/rust-lang/rust/issues/95513
#![allow(unused_crate_dependencies)]

use libcnb_test::BuildpackReference;

mod smoke;
mod ux;

pub(crate) fn default_buildpacks() -> Vec<BuildpackReference> {
    vec![
        BuildpackReference::Other(String::from("heroku/jvm")),
        BuildpackReference::CurrentCrate,
        // Using an explicit version from Docker Hub to prevent failures when there
        // are multiple Procfile buildpack versions in the builder image.
        BuildpackReference::Other(String::from("docker://docker.io/heroku/procfile-cnb:2.0.1")),
    ]
}
