use buildpacks_jvm_shared::default_on_not_found;
use libcnb::Env;
use std::fs;
use std::path::Path;
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
    CouldNotReadSystemProperties(std::io::Error),
    CouldNotParseSystemProperties(java_properties::PropertiesError),
    InvalidPreTaskList(shell_words::ParseError),
    InvalidTaskList(shell_words::ParseError),
    InvalidSbtClean(ParseBoolError),
    InvalidAvailableAtLaunch(ParseBoolError),
}

pub(crate) fn read_sbt_buildpack_configuration(
    app_dir: &Path,
    env: &Env,
) -> Result<SbtBuildpackConfiguration, ReadSbtBuildpackConfigurationError> {
    let system_properties = default_on_not_found(fs::read(app_dir.join("system.properties")))
        .map_err(ReadSbtBuildpackConfigurationError::CouldNotReadSystemProperties)
        .and_then(|file_contents| {
            java_properties::read(&file_contents[..])
                .map_err(ReadSbtBuildpackConfigurationError::CouldNotParseSystemProperties)
        })?;

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
    use super::ReadSbtBuildpackConfigurationError;
    use libcnb::Env;
    use std::collections::HashMap;
    use std::ffi::{OsStr, OsString};
    use std::fs::{create_dir, write, File};
    use std::io::BufWriter;
    use std::os::unix::ffi::OsStrExt;
    use tempfile::{tempdir, TempDir};

    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
            }
        }
    }

    fn set_sbt_version(app_dir: &TempDir, version: &str) {
        let sbt_project_path = app_dir.path().join("project");
        create_dir(&sbt_project_path).unwrap();
        let contents = format!("sbt.version={version}");
        write(sbt_project_path.join("build.properties"), contents).unwrap();
    }

    fn set_system_properties(app_dir: &TempDir, properties: HashMap<&str, &str>) {
        let property_file = File::create(app_dir.path().join("system.properties")).unwrap();
        let writer = BufWriter::new(property_file);
        let properties = properties
            .into_iter()
            .map(|(key, val)| (key.to_string(), val.to_string()))
            .collect();
        java_properties::write(writer, &properties).unwrap();
    }

    fn invalid_unicode_os_string() -> OsString {
        let invalid_unicode_sequence = [0x66, 0x6f, 0x80, 0x6f];
        OsStr::from_bytes(&invalid_unicode_sequence[..]).to_os_string()
    }

    #[test]
    fn create_build_config_when_sbt_project_is_not_configured() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_project, None);
    }

    #[test]
    fn create_build_config_when_sbt_project_is_configured_from_property() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(
            &app_dir,
            HashMap::from([("sbt.project", "testProjectName")]),
        );
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_project, Some(String::from("testProjectName")));
    }

    #[test]
    fn create_build_config_when_sbt_project_is_configured_from_environment() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        env.insert("SBT_PROJECT", "testProjectName");
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_project, Some(String::from("testProjectName")));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_project_is_configured_from_environment_with_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        env.insert("SBT_PROJECT", invalid_unicode_os_string());

        assert_eq!(
            read_sbt_buildpack_configuration(app_dir.path(), &env)
                .unwrap()
                .sbt_project,
            Some(invalid_unicode_os_string().to_string_lossy().to_string())
        );
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_not_configured() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_pre_tasks, None);
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_configured_from_system_properties() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.pre-tasks", "task1 task2")]));
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task1"), String::from("task2")];
        assert_eq!(config.sbt_pre_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_configured_from_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1 task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task1"), String::from("task2")];
        assert_eq!(config.sbt_pre_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_prefers_system_property_over_env_for_sbt_pre_tasks() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1 task2"));
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.pre-tasks", "task3 task4")]));
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task3"), String::from("task4")];
        assert_eq!(config.sbt_pre_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_property_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_system_properties(
            &app_dir,
            HashMap::from([("sbt.pre-tasks", "task1\" task2")]),
        );
        set_sbt_version(&app_dir, "1.8.2");
        let err = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap_err();
        assert_err!(
            err,
            ReadSbtBuildpackConfigurationError::InvalidPreTaskList(_)
        );
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_environment_variable_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1\" task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap_err();
        assert_err!(
            err,
            ReadSbtBuildpackConfigurationError::InvalidPreTaskList(_)
        );
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_environment_variable_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        assert_eq!(
            read_sbt_buildpack_configuration(app_dir.path(), &env)
                .unwrap()
                .sbt_pre_tasks,
            Some(vec![invalid_unicode_os_string()
                .to_string_lossy()
                .to_string()])
        );
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_not_configured() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, None);
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_system_properties() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.clean", "true")]));
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, Some(true));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_system_properties_and_value_is_not_parsable_as_boolean(
    ) {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.clean", "")]));
        let err = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ReadSbtBuildpackConfigurationError::InvalidSbtClean(_));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", OsString::from("true"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, Some(true));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env_and_value_is_not_parsable_as_boolean(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", OsString::from("blah"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ReadSbtBuildpackConfigurationError::InvalidSbtClean(_));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env_and_value_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        let err = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ReadSbtBuildpackConfigurationError::InvalidSbtClean(_));
    }

    #[test]
    fn create_build_config_when_sbt_clean_prefers_system_property_over_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", OsString::from("false"));
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.clean", "true")]));
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, Some(true));
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_not_configured() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_tasks, None);
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_configured_from_system_properties() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.tasks", "task1 task2")]));
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task1"), String::from("task2")];
        assert_eq!(config.sbt_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_configured_from_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", OsString::from("task1 task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task1"), String::from("task2")];
        assert_eq!(config.sbt_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_prefers_system_property_over_env_for_sbt_tasks() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", OsString::from("task1 task2"));
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.tasks", "task3 task4")]));
        let config = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task3"), String::from("task4")];
        assert_eq!(config.sbt_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_property_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_system_properties(&app_dir, HashMap::from([("sbt.tasks", "task1\" task2")]));
        set_sbt_version(&app_dir, "1.8.2");
        let err = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ReadSbtBuildpackConfigurationError::InvalidTaskList(_));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_environment_variable_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", OsString::from("task1\" task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = read_sbt_buildpack_configuration(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ReadSbtBuildpackConfigurationError::InvalidTaskList(_));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_environment_variable_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        assert_eq!(
            read_sbt_buildpack_configuration(app_dir.path(), &env)
                .unwrap()
                .sbt_tasks,
            Some(vec![invalid_unicode_os_string()
                .to_string_lossy()
                .to_string()])
        );
    }
}
