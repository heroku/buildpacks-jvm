use crate::util::validate_sha256;
use crate::{HerokuMetricsAgentMetadata, OpenJdkBuildpack, OpenJdkBuildpackError};
use libcnb::build::BuildContext;
use libcnb::data::layer_name;
use libcnb::layer::{
    InspectExistingAction, InvalidMetadataAction, LayerDefinition, LayerDefinitionResult,
};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::{additional_buildpack_binary_path, Buildpack};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct HerokuMetricsAgentLayerMetadata {
    source: HerokuMetricsAgentMetadata,
}

pub(crate) fn handle(
    context: &BuildContext<OpenJdkBuildpack>,
) -> libcnb::Result<(), OpenJdkBuildpackError> {
    let layer = context.execute_layer_definition(
        layer_name!("heroku_metrics_agent"),
        LayerDefinition {
            build: false,
            launch: true,
            cache: true,
            invalid_metadata: &|_| InvalidMetadataAction::DeleteLayer,
            inspect_existing: &|metadata: &HerokuMetricsAgentLayerMetadata, _| {
                if metadata.source == context.buildpack_descriptor.metadata.heroku_metrics_agent {
                    InspectExistingAction::Keep
                } else {
                    InspectExistingAction::Delete
                }
            },
        },
    )?;

    if let LayerDefinitionResult::Empty { layer_data, .. } = layer {
        let agent_jar_path = layer_data.path.join("heroku-metrics-agent.jar");

        let metrics_agent_metadata = &context.buildpack_descriptor.metadata.heroku_metrics_agent;

        libherokubuildpack::download::download_file(&metrics_agent_metadata.url, &agent_jar_path)
            .map_err(OpenJdkBuildpackError::MetricsAgentDownloadError)?;

        validate_sha256(&agent_jar_path, &metrics_agent_metadata.sha256)
            .map_err(OpenJdkBuildpackError::MetricsAgentSha256ValidationError)?;

        libcnb::layer::replace_execd_programs(
            &[(
                "heroku_metrics_agent_setup",
                &additional_buildpack_binary_path!("heroku_metrics_agent_setup"),
            )],
            &layer_data.path,
        )?;

        libcnb::layer::replace_env(
            LayerEnv::new().chainable_insert(
                Scope::All,
                ModificationBehavior::Override,
                "HEROKU_METRICS_AGENT_PATH",
                agent_jar_path,
            ),
            &layer_data.path,
        )?;
    }

    Ok(())
}
