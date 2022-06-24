// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// Re-disable pedantic lints that are too noisy/unwanted.
#![allow(clippy::module_name_repetitions)]

use crate::common::project_toml_salesforce_type_is_function;
use crate::error::{handle_buildpack_error, JvmFunctionInvokerBuildpackError};
use crate::layers::bundle::BundleLayer;
use crate::layers::opt::OptLayer;
use crate::layers::runtime::RuntimeLayer;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::buildpack_main;
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::launch::{Launch, ProcessBuilder};
use libcnb::data::{layer_name, process_type};
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::layer_env::Scope;
use libcnb::{Buildpack, Env};
use libherokubuildpack::{log_header, log_info, on_error_heroku};
use serde::Deserialize;

mod common;
mod error;
mod layers;

pub struct JvmFunctionInvokerBuildpack;

#[derive(Deserialize, Debug)]
pub struct JvmFunctionInvokerBuildpackMetadata {
    runtime: JvmFunctionInvokerBuildpackRuntimeMetadata,
}

#[derive(Deserialize, Debug)]
pub struct JvmFunctionInvokerBuildpackRuntimeMetadata {
    url: String,
    sha256: String,
}

impl Buildpack for JvmFunctionInvokerBuildpack {
    type Platform = GenericPlatform;
    type Metadata = JvmFunctionInvokerBuildpackMetadata;
    type Error = JvmFunctionInvokerBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        let function_toml_path = context.app_dir.join("function.toml");
        let project_toml_path = context.app_dir.join("project.toml");

        if function_toml_path.exists()
            || project_toml_salesforce_type_is_function(&project_toml_path)
        {
            DetectResultBuilder::pass()
                .build_plan(
                    BuildPlanBuilder::new()
                        .requires("jdk")
                        .requires("jvm-application")
                        .build(),
                )
                .build()
        } else {
            DetectResultBuilder::fail().build()
        }
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        context.handle_layer(layer_name!("opt"), OptLayer)?;

        log_header("Installing Java function runtime");
        let runtime_layer = context.handle_layer(layer_name!("runtime"), RuntimeLayer)?;
        log_info("Function runtime installation successful");

        context.handle_layer(
            layer_name!("bundle"),
            BundleLayer {
                env: runtime_layer.env.apply(Scope::Build, &Env::new()),
            },
        )?;

        BuildResultBuilder::new()
            .launch(
                Launch::new().process(
                    ProcessBuilder::new(process_type!("web"), layers::opt::JVM_RUNTIME_SCRIPT_NAME)
                        .default(true)
                        .build(),
                ),
            )
            .build()
    }

    fn on_error(&self, error: libcnb::Error<Self::Error>) {
        on_error_heroku(handle_buildpack_error, error)
    }
}

buildpack_main!(JvmFunctionInvokerBuildpack);
