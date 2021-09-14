// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// Re-disable pedantic lints that are too noisy/unwanted.
#![allow(clippy::module_name_repetitions)]

use std::fmt::Debug;

use crate::error::handle_buildpack_error;
use libcnb::cnb_runtime;
use libherokubuildpack::HerokuBuildpackErrorHandler;
use serde::Deserialize;

mod build;
mod detect;
mod error;
mod layers;

fn main() {
    cnb_runtime(
        detect::detect,
        build::build,
        HerokuBuildpackErrorHandler::new(Box::new(handle_buildpack_error)),
    );
}

#[derive(Deserialize, Debug)]
pub struct JvmFunctionInvokerBuildpackMetadata {
    runtime: JvmFunctionInvokerBuildpackRuntimeMetadata,
}

#[derive(Deserialize, Debug)]
pub struct JvmFunctionInvokerBuildpackRuntimeMetadata {
    url: String,
    sha256: String,
}
