use libcnb::Env;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
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
pub(crate) enum SbtBuildpackConfigurationError {
    InvalidPreTaskList(shell_words::ParseError),
    InvalidTaskList(shell_words::ParseError),
    InvalidSbtClean(ParseBoolError),
    InvalidAvailableAtLaunch(ParseBoolError),
}

pub(crate) fn create_build_config<P: Into<PathBuf>>(
    app_dir: P,
    env: &Env,
) -> Result<SbtBuildpackConfiguration, SbtBuildpackConfigurationError> {
    let app_dir = app_dir.into();
    let properties = read_system_properties(&app_dir);

    Ok(SbtBuildpackConfiguration {
        sbt_project: properties.get("sbt.project").cloned().or(env
            .get("SBT_PROJECT")
            .map(|os_string| os_string.to_string_lossy().to_string())),
        sbt_pre_tasks: properties
            .get("sbt.pre-tasks")
            .cloned()
            .or(env
                .get("SBT_PRE_TASKS")
                .map(|os_string| os_string.to_string_lossy().to_string()))
            .map(|string| shell_words::split(&string))
            .transpose()
            .map_err(SbtBuildpackConfigurationError::InvalidPreTaskList)?,
        sbt_tasks: properties
            .get("sbt.tasks")
            .cloned()
            .or(env
                .get("SBT_TASKS")
                .map(|os_string| os_string.to_string_lossy().to_string()))
            .map(|string| shell_words::split(&string))
            .transpose()
            .map_err(SbtBuildpackConfigurationError::InvalidTaskList)?,
        sbt_clean: properties
            .get("sbt.clean")
            .cloned()
            .or(env
                .get("SBT_CLEAN")
                .map(|os_string| os_string.to_string_lossy().to_string()))
            .map(|string| string.parse())
            .transpose()
            .map_err(SbtBuildpackConfigurationError::InvalidSbtClean)?,
        sbt_available_at_launch: properties
            .get("sbt.available-at-launch")
            .cloned()
            .or(env
                .get("SBT_AVAILABLE_AT_LAUNCH")
                .map(|os_string| os_string.to_string_lossy().to_string()))
            .map(|string| string.parse())
            .transpose()
            .map_err(SbtBuildpackConfigurationError::InvalidAvailableAtLaunch)?,
    })
}

fn read_system_properties(app_dir: &Path) -> HashMap<String, String> {
    File::open(app_dir.join("system.properties"))
        .map(|file| java_properties::read(file).unwrap_or_default())
        .unwrap_or_default()
}

#[cfg(test)]
mod test {
    use super::create_build_config;
    use super::SbtBuildpackConfigurationError;
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
        let config = create_build_config(app_dir.path(), &env).unwrap();
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
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_project, Some(String::from("testProjectName")));
    }

    #[test]
    fn create_build_config_when_sbt_project_is_configured_from_environment() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        env.insert("SBT_PROJECT", "testProjectName");
        let config = create_build_config(app_dir.path(), &env).unwrap();
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
            create_build_config(app_dir.path(), &env)
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
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_pre_tasks, None);
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_configured_from_system_properties() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.pre-tasks", "task1 task2")]));
        let config = create_build_config(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task1"), String::from("task2")];
        assert_eq!(config.sbt_pre_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_configured_from_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1 task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
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
        let config = create_build_config(app_dir.path(), &env).unwrap();
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
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, SbtBuildpackConfigurationError::InvalidPreTaskList(_));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_environment_variable_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1\" task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, SbtBuildpackConfigurationError::InvalidPreTaskList(_));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_environment_variable_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        assert_eq!(
            create_build_config(app_dir.path(), &env)
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
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, None);
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_system_properties() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.clean", "true")]));
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, Some(true));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_system_properties_and_value_is_not_parsable_as_boolean(
    ) {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.clean", "")]));
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, SbtBuildpackConfigurationError::InvalidSbtClean(_));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", OsString::from("true"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, Some(true));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env_and_value_is_not_parsable_as_boolean(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", OsString::from("blah"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, SbtBuildpackConfigurationError::InvalidSbtClean(_));
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env_and_value_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, SbtBuildpackConfigurationError::InvalidSbtClean(_));
    }

    #[test]
    fn create_build_config_when_sbt_clean_prefers_system_property_over_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", OsString::from("false"));
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.clean", "true")]));
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, Some(true));
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_not_configured() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_tasks, None);
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_configured_from_system_properties() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.tasks", "task1 task2")]));
        let config = create_build_config(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task1"), String::from("task2")];
        assert_eq!(config.sbt_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_when_sbt_tasks_is_configured_from_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", OsString::from("task1 task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
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
        let config = create_build_config(app_dir.path(), &env).unwrap();
        let expected_tasks: Vec<String> = vec![String::from("task3"), String::from("task4")];
        assert_eq!(config.sbt_tasks, Some(expected_tasks));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_property_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_system_properties(&app_dir, HashMap::from([("sbt.tasks", "task1\" task2")]));
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, SbtBuildpackConfigurationError::InvalidTaskList(_));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_environment_variable_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", OsString::from("task1\" task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, SbtBuildpackConfigurationError::InvalidTaskList(_));
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_environment_variable_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        assert_eq!(
            create_build_config(app_dir.path(), &env).unwrap().sbt_tasks,
            Some(vec![invalid_unicode_os_string()
                .to_string_lossy()
                .to_string()])
        );
    }
}
