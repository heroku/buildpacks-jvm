// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// Re-disable pedantic lints that are too noisy/unwanted.
#![allow(clippy::module_name_repetitions)]

use std::fmt::Debug;

use crate::common::project_toml_salesforce_type_is_function;
use crate::error::{handle_buildpack_error, JvmFunctionInvokerBuildpackError};
use crate::layers::bundle::BundleLayer;
use crate::layers::opt::OptLayer;
use crate::layers::runtime::RuntimeLayer;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::launch::{Launch, Process};
use libcnb::data::layer_name;
use libcnb::data::process_type;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::layer_env::TargetLifecycle;
use libcnb::{buildpack_main, Error};
use libcnb::{Buildpack, Env};
use libherokubuildpack::{handle_error_heroku, log_header, log_info};
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
                env: runtime_layer.env.apply(TargetLifecycle::Build, &Env::new()),
            },
        )?;

        BuildResultBuilder::new()
            .launch(Launch::default().process(Process::new(
                process_type!("web"),
                String::from(layers::opt::JVM_RUNTIME_SCRIPT_NAME),
                Vec::<String>::new(),
                false,
                true,
            )))
            .build()
    }

    fn handle_error(&self, error: Error<Self::Error>) -> i32 {
        handle_error_heroku(handle_buildpack_error, error)
    }
}

buildpack_main!(JvmFunctionInvokerBuildpack);
