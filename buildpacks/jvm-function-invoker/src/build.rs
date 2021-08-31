use std::path::PathBuf;

use libcnb::{BuildContext, GenericPlatform};
use libcnb::data::launch::{Launch, Process};
use libcnb::layer_lifecycle::execute_layer_lifecycle;

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;
use crate::layers::bundle::BundleLayerLifecycle;
use crate::layers::opt::OptLayerLifecycle;
use crate::layers::runtime::RuntimeLayerLifecycle;

pub fn build(
    context: BuildContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
) -> Result<(), libcnb::Error<JvmFunctionInvokerBuildpackError>> {
    let run_sh_path: PathBuf = execute_layer_lifecycle("opt", OptLayerLifecycle {}, &context)?;

    let runtime_jar_path: PathBuf =
        execute_layer_lifecycle("runtime", RuntimeLayerLifecycle {}, &context)?;

    let bundle_path = execute_layer_lifecycle(
        "bundle",
        BundleLayerLifecycle {
            invoker_jar_path: runtime_jar_path.clone(),
        },
        &context,
    )?;

    let launch = Launch::default().process(Process::new(
        "web",
        run_sh_path.to_string_lossy(),
        vec![
            runtime_jar_path.to_string_lossy(),
            bundle_path.to_string_lossy(),
        ],
        false,
    )?);

    context
        .write_launch(launch)
        .map_err(JvmFunctionInvokerBuildpackError::CouldNotWriteLaunchToml)?;

    Ok(())
}
