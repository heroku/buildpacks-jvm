use crate::{OpenJdkBuildpack, OpenJdkBuildpackError};
use libcnb::additional_buildpack_binary_path;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::UncachedLayerDefinition;

pub(crate) fn handle_runtime_layer(
    context: &BuildContext<OpenJdkBuildpack>,
) -> libcnb::Result<(), OpenJdkBuildpackError> {
    let layer_ref = context.uncached_layer(
        layer_name!("runtime"),
        UncachedLayerDefinition {
            build: false,
            launch: true,
        },
    )?;

    layer_ref.write_exec_d_programs([(
        "heroku_database_env_var_rewrite",
        additional_buildpack_binary_path!("heroku_database_env_var_rewrite"),
    )])
}
