use libcnb::Env;
use std::collections::HashMap;
use std::str::ParseBoolError;

#[derive(Debug)]
#[allow(clippy::struct_field_names)]
pub(crate) struct SbtBuildpackConfiguration {
    pub(crate) sbt_project: Option<String>,
    pub(crate) sbt_pre_tasks: Option<Vec<String>>,
    pub(crate) sbt_tasks: Option<Vec<String>>,
    pub(crate) sbt_clean: Option<bool>,
    pub(crate) sbt_available_at_launch: Option<bool>,
}

#[derive(Debug)]
pub(crate) enum ReadSbtBuildpackConfigurationError {
    InvalidPreTaskList(shell_words::ParseError),
    InvalidTaskList(shell_words::ParseError),
    InvalidSbtClean(ParseBoolError),
    InvalidAvailableAtLaunch(ParseBoolError),
}

pub(crate) fn read_sbt_buildpack_configuration(
    system_properties: &HashMap<String, String>,
    env: &Env,
) -> Result<SbtBuildpackConfiguration, ReadSbtBuildpackConfigurationError> {
    Ok(SbtBuildpackConfiguration {
        sbt_project: system_properties
            .get("sbt.project")
            .cloned()
            .or(env.get_string_lossy("SBT_PROJECT")),
        sbt_pre_tasks: system_properties
            .get("sbt.pre-tasks")
            .cloned()
            .or(env.get_string_lossy("SBT_PRE_TASKS"))
            .map(|string| shell_words::split(&string))
            .transpose()
            .map_err(ReadSbtBuildpackConfigurationError::InvalidPreTaskList)?,
        sbt_tasks: system_properties
            .get("sbt.tasks")
            .cloned()
            .or(env.get_string_lossy("SBT_TASKS"))
            .map(|string| shell_words::split(&string))
            .transpose()
            .map_err(ReadSbtBuildpackConfigurationError::InvalidTaskList)?,
        sbt_clean: system_properties
            .get("sbt.clean")
            .cloned()
            .or(env.get_string_lossy("SBT_CLEAN"))
            .map(|string| string.parse())
            .transpose()
            .map_err(ReadSbtBuildpackConfigurationError::InvalidSbtClean)?,
        sbt_available_at_launch: system_properties
            .get("sbt.available-at-launch")
            .cloned()
            .or(env.get_string_lossy("SBT_AVAILABLE_AT_LAUNCH"))
            .map(|string| string.parse())
            .transpose()
            .map_err(ReadSbtBuildpackConfigurationError::InvalidAvailableAtLaunch)?,
    })
}
