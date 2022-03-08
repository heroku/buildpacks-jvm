use crate::util::validate_sha256;
use crate::{
    HerokuMetricsAgentMetadata, OpenJdkBuildpack, OpenJdkBuildpackError,
    JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER, JAVA_TOOL_OPTIONS_ENV_VAR_NAME,
};
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::layer::{ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use libcnb::Buildpack;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;

pub struct MetricsAgentLayer;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MetricsAgentLayerMetadata {
    source: HerokuMetricsAgentMetadata,
}

impl Layer for MetricsAgentLayer {
    type Buildpack = OpenJdkBuildpack;
    type Metadata = MetricsAgentLayerMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: false,
            launch: true,
            cache: true,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        libherokubuildpack::log_header("Installing Heroku JVM metrics agent");

        let agent_jar_path = layer_path.join("heroku-metrics-agent.jar");

        let metrics_agent_metadata = &context.buildpack_descriptor.metadata.heroku_metrics_agent;

        libherokubuildpack::download_file(&metrics_agent_metadata.url, &agent_jar_path)
            .map_err(OpenJdkBuildpackError::MetricsAgentDownloadError)?;

        validate_sha256(&agent_jar_path, &metrics_agent_metadata.sha256)
            .map_err(OpenJdkBuildpackError::MetricsAgentSha256ValidationError)?;

        LayerResultBuilder::new(MetricsAgentLayerMetadata {
            source: (*metrics_agent_metadata).clone(),
        })
        .env(
            LayerEnv::new()
                .chainable_insert(
                    Scope::Launch,
                    ModificationBehavior::Delimiter,
                    JAVA_TOOL_OPTIONS_ENV_VAR_NAME,
                    JAVA_TOOL_OPTIONS_ENV_VAR_DELIMITER,
                )
                .chainable_insert(
                    Scope::Launch,
                    ModificationBehavior::Prepend,
                    JAVA_TOOL_OPTIONS_ENV_VAR_NAME,
                    format!("-javaagent:{}", agent_jar_path.to_string_lossy()),
                ),
        )
        .build()
    }

    fn existing_layer_strategy(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        if layer_data.content_metadata.metadata.source
            == context.buildpack_descriptor.metadata.heroku_metrics_agent
        {
            Ok(ExistingLayerStrategy::Keep)
        } else {
            Ok(ExistingLayerStrategy::Recreate)
        }
    }
}
