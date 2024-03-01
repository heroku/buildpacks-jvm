use crate::{OpenJdkBuildpack, OpenJdkBuildpackError};
use libcnb::additional_buildpack_binary_path;
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::UncachedLayerDefinition;

pub(crate) fn handle(
    context: &BuildContext<OpenJdkBuildpack>,
) -> libcnb::Result<(), OpenJdkBuildpackError> {
    let layer = context.execute_uncached_layer_definition(
        layer_name!("runtime"),
        UncachedLayerDefinition {
            build: false,
            launch: true,
        },
    )?;

    libcnb::layer::replace_execd_programs(
        &[(
            "heroku_database_env_var_rewrite",
            &additional_buildpack_binary_path!("heroku_database_env_var_rewrite"),
        )],
        &layer.path,
    )?;

    Ok(())
}
