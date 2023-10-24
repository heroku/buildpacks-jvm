//! Bundle all integration tests into one binary to:
//! - Reduce compile times
//! - Reduce required disk space
//! - Increase parallelism
//!
//! See: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Implications

use buildpacks_jvm_shared_test::DEFAULT_INTEGRATION_TEST_BUILDER;
use libcnb_test::{BuildConfig, BuildpackReference};

mod automatic_process_type;
mod caching;
mod customization;
mod misc;
mod polyglot;
mod settings_xml;
mod smoke;
mod versions;

pub(crate) fn default_config() -> BuildConfig {
    BuildConfig::new(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/simple-http-service",
    )
    .buildpacks(default_buildpacks())
    .clone()
}

pub(crate) fn default_buildpacks() -> Vec<BuildpackReference> {
    vec![
        BuildpackReference::Other(String::from("heroku/jvm")),
        BuildpackReference::CurrentCrate,
    ]
}
