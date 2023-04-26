use libcnb::Env;
use std::collections::HashMap;
use std::str::ParseBoolError;

#[derive(Debug)]
pub(crate) struct SbtBuildpackConfiguration {
    pub(crate) sbt_project: Option<String>,
    pub(crate) sbt_pre_tasks: Option<Vec<String>>,
    pub(crate) sbt_tasks: Option<Vec<String>>,
    pub(crate) sbt_clean: Option<bool>,
    pub(crate) sbt_available_at_launch: Option<bool>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
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
        sbt_project: system_properties.get("sbt.project").cloned().or(env
            .get("SBT_PROJECT")
            .map(|os_string| os_string.to_string_lossy().to_string())),
        sbt_pre_tasks: system_properties
            .get("sbt.pre-tasks")
            .cloned()
            .or(env
                .get("SBT_PRE_TASKS")
                .map(|os_string| os_string.to_string_lossy().to_string()))
            .map(|string| shell_words::split(&string))
            .transpose()
            .map_err(ReadSbtBuildpackConfigurationError::InvalidPreTaskList)?,
        sbt_tasks: system_properties
            .get("sbt.tasks")
            .cloned()
            .or(env
                .get("SBT_TASKS")
                .map(|os_string| os_string.to_string_lossy().to_string()))
            .map(|string| shell_words::split(&string))
            .transpose()
            .map_err(ReadSbtBuildpackConfigurationError::InvalidTaskList)?,
        sbt_clean: system_properties
            .get("sbt.clean")
            .cloned()
            .or(env
                .get("SBT_CLEAN")
                .map(|os_string| os_string.to_string_lossy().to_string()))
            .map(|string| string.parse())
            .transpose()
            .map_err(ReadSbtBuildpackConfigurationError::InvalidSbtClean)?,
        sbt_available_at_launch: system_properties
            .get("sbt.available-at-launch")
            .cloned()
            .or(env
                .get("SBT_AVAILABLE_AT_LAUNCH")
                .map(|os_string| os_string.to_string_lossy().to_string()))
            .map(|string| string.parse())
            .transpose()
            .map_err(ReadSbtBuildpackConfigurationError::InvalidAvailableAtLaunch)?,
    })
}

#[cfg(test)]
mod test {
    use super::read_sbt_buildpack_configuration;
    use crate::build_configuration::ReadSbtBuildpackConfigurationError;
    use libcnb::Env;
    use std::collections::HashMap;
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    fn invalid_unicode_os_string() -> OsString {
        OsString::from_vec(vec![0x66, 0x6f, 0x80, 0x6f])
    }

    #[test]
    fn create_build_config_when_sbt_project_is_not_configured() {
        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::default(), &Env::new())
                .unwrap()
                .sbt_project,
            None
        );
    }

    #[test]
    fn create_build_config_when_sbt_project_is_configured_from_property() {
        assert_eq!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.project"), String::from("testProjectName"))]),
                &Env::new()
            )
            .unwrap()
            .sbt_project,
            Some(String::from("testProjectName"))
        );
    }

    #[test]
    fn create_build_config_when_sbt_project_is_configured_from_environment() {
        let mut env = Env::new();
        env.insert("SBT_PROJECT", "testProjectName");

        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::default(), &env)
                .unwrap()
                .sbt_project,
            Some(String::from("testProjectName"))
        );
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_project_is_configured_from_environment_with_non_unicode_bytes(
    ) {
        let mut env = Env::new();
        env.insert("SBT_PROJECT", invalid_unicode_os_string());

        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::default(), &env)
                .unwrap()
                .sbt_project,
            Some(invalid_unicode_os_string().to_string_lossy().to_string())
        );
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_not_configured() {
        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::new(), &Env::new())
                .unwrap()
                .sbt_pre_tasks,
            None
        );
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_configured_from_system_properties() {
        assert_eq!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.pre-tasks"), String::from("task1 task2"))]),
                &Env::new()
            )
            .unwrap()
            .sbt_pre_tasks,
            Some(vec![String::from("task1"), String::from("task2")])
        );
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_configured_from_env() {
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1 task2"));

        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::new(), &env)
                .unwrap()
                .sbt_pre_tasks,
            Some(vec![String::from("task1"), String::from("task2")])
        );
    }

    #[test]
    fn create_build_config_prefers_system_property_over_env_for_sbt_pre_tasks() {
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1 task2"));

        assert_eq!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.pre-tasks"), String::from("task3 task4"))]),
                &env
            )
            .unwrap()
            .sbt_pre_tasks,
            Some(vec![String::from("task3"), String::from("task4")])
        );
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_property_cannot_be_split() {
        assert!(matches!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.pre-tasks"), String::from("task1\" task2"))]),
                &Env::new()
            ),
            Err(ReadSbtBuildpackConfigurationError::InvalidPreTaskList(_))
        ));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_environment_variable_cannot_be_split() {
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1\" task2"));

        assert!(matches!(
            read_sbt_buildpack_configuration(&HashMap::default(), &env),
            Err(ReadSbtBuildpackConfigurationError::InvalidPreTaskList(_))
        ));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_environment_variable_contains_non_unicode_bytes(
    ) {
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", invalid_unicode_os_string());

        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::new(), &env)
                .unwrap()
                .sbt_pre_tasks,
            Some(vec![invalid_unicode_os_string()
                .to_string_lossy()
                .to_string()])
        );
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_not_configured() {
        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::new(), &Env::new())
                .unwrap()
                .sbt_clean,
            None
        );
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_system_properties() {
        assert_eq!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.clean"), String::from("true"))]),
                &Env::new(),
            )
            .unwrap()
            .sbt_clean,
            Some(true)
        );
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_system_properties_and_value_is_not_parsable_as_boolean(
    ) {
        assert!(matches!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.clean"), String::new())]),
                &Env::new()
            ),
            Err(ReadSbtBuildpackConfigurationError::InvalidSbtClean(_))
        ));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env() {
        let mut env = Env::new();
        env.insert("SBT_CLEAN", "true");

        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::default(), &env)
                .unwrap()
                .sbt_clean,
            Some(true)
        );
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env_and_value_is_not_parsable_as_boolean(
    ) {
        let mut env = Env::new();
        env.insert("SBT_CLEAN", "blah");

        assert!(matches!(
            read_sbt_buildpack_configuration(&HashMap::new(), &env),
            Err(ReadSbtBuildpackConfigurationError::InvalidSbtClean(_))
        ));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env_and_value_contains_non_unicode_bytes(
    ) {
        let mut env = Env::new();
        env.insert("SBT_CLEAN", invalid_unicode_os_string());

        assert!(matches!(
            read_sbt_buildpack_configuration(&HashMap::default(), &env),
            Err(ReadSbtBuildpackConfigurationError::InvalidSbtClean(_))
        ));
    }

    #[test]
    fn create_build_config_when_sbt_clean_prefers_system_property_over_env() {
        let mut env = Env::new();
        env.insert("SBT_CLEAN", OsString::from("false"));

        assert_eq!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.clean"), String::from("true"))]),
                &env,
            )
            .unwrap()
            .sbt_clean,
            Some(true)
        );
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_not_configured() {
        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::new(), &Env::new())
                .unwrap()
                .sbt_tasks,
            None
        );
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_configured_from_system_properties() {
        assert_eq!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.tasks"), String::from("task1 task2"))]),
                &Env::new(),
            )
            .unwrap()
            .sbt_tasks,
            Some(vec![String::from("task1"), String::from("task2")])
        );
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_configured_from_env() {
        let mut env = Env::new();
        env.insert("SBT_TASKS", "task1 task2");

        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::new(), &env)
                .unwrap()
                .sbt_tasks,
            Some(vec![String::from("task1"), String::from("task2")])
        );
    }

    #[test]
    fn create_build_config_prefers_system_property_over_env_for_sbt_tasks() {
        let mut env = Env::new();
        env.insert("SBT_TASKS", "task1 task2");

        assert_eq!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.tasks"), String::from("task3 task4"))]),
                &env,
            )
            .unwrap()
            .sbt_tasks,
            Some(vec![String::from("task3"), String::from("task4")])
        );
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_property_cannot_be_split() {
        assert!(matches!(
            read_sbt_buildpack_configuration(
                &HashMap::from([(String::from("sbt.tasks"), String::from("task1\" task2"))]),
                &Env::new(),
            ),
            Err(ReadSbtBuildpackConfigurationError::InvalidTaskList(_))
        ));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_environment_variable_cannot_be_split() {
        let mut env = Env::new();
        env.insert("SBT_TASKS", "task1\" task2");

        assert!(matches!(
            read_sbt_buildpack_configuration(&HashMap::new(), &env),
            Err(ReadSbtBuildpackConfigurationError::InvalidTaskList(_))
        ));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_environment_variable_contains_non_unicode_bytes(
    ) {
        let mut env = Env::new();
        env.insert("SBT_TASKS", invalid_unicode_os_string());

        assert_eq!(
            read_sbt_buildpack_configuration(&HashMap::new(), &env)
                .unwrap()
                .sbt_tasks,
            Some(vec![invalid_unicode_os_string()
                .to_string_lossy()
                .to_string()])
        );
    }
}
