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
