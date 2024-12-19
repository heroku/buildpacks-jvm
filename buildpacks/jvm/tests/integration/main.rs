//! Bundle all integration tests into one binary to:
//! - Reduce compile times
//! - Reduce required disk space
//! - Increase parallelism
//!
//! See: <https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Implications>

// Required due to: https://github.com/rust-lang/rust/issues/95513
#![allow(unused_crate_dependencies)]

use libcnb_test::BuildConfig;
use std::path::Path;

mod overlay;
mod versions;

fn default_build_config(fixture_path: impl AsRef<Path>) -> BuildConfig {
    let builder = builder();

    // TODO: Once Pack build supports `--platform` and libcnb-test adjusted accordingly, change this
    // to allow configuring the target arch independently of the builder name (eg via env var).
    let target_triple = match builder.as_str() {
        // Compile the buildpack for ARM64 iff the builder supports multi-arch and the host is ARM64.
        "heroku/builder:24" if cfg!(target_arch = "aarch64") => "aarch64-unknown-linux-musl",
        _ => "x86_64-unknown-linux-musl",
    };

    let mut config = BuildConfig::new(&builder, fixture_path);
    config.target_triple(target_triple);
    config
}

fn builder() -> String {
    std::env::var("INTEGRATION_TEST_BUILDER").unwrap_or(String::from(DEFAULT_BUILDER))
}

const DEFAULT_BUILDER: &str = "heroku/builder:24";
