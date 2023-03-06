use crate::errors::ScalaBuildpackError;
use crate::errors::ScalaBuildpackError::{
    CouldNotConvertEnvironmentValueIntoString, CouldNotParseBooleanFromEnvironment,
    CouldNotParseBooleanFromProperty, CouldNotParseListConfigurationFromEnvironment,
    CouldNotParseListConfigurationFromProperty, CouldNotParseListConfigurationFromSbtOptsFile,
    CouldNotReadSbtOptsFile, InvalidSbtPropertiesFile, MissingDeclaredSbtVersion,
    MissingSbtBuildPropertiesFile, SbtPropertiesFileReadError, SbtVersionNotInSemverFormat,
    UnsupportedSbtVersion,
};
use libcnb::Env;
use semver::{Version, VersionReq};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub(crate) struct BuildConfiguration {
    pub(crate) sbt_project: Option<String>,
    pub(crate) sbt_pre_tasks: Option<Vec<String>>,
    pub(crate) sbt_tasks: Option<Vec<String>>,
    pub(crate) sbt_clean: Option<bool>,
    pub(crate) sbt_opts: Option<Vec<String>>,
    pub(crate) sbt_available_at_launch: Option<bool>,
    pub(crate) sbt_version: Version,
}

pub(crate) fn create_build_config<P: Into<PathBuf>>(
    app_dir: P,
    env: &Env,
) -> Result<BuildConfiguration, ScalaBuildpackError> {
    let app_dir = app_dir.into();
    let sbt_opts_file = app_dir.join(".sbtopts");
    let properties = read_system_properties(&app_dir);
    Ok(BuildConfiguration {
        sbt_project: read_string_config("sbt.project", &properties, "SBT_PROJECT", env)?,
        sbt_pre_tasks: read_string_list_config("sbt.pre-tasks", &properties, "SBT_PRE_TASKS", env)?,
        sbt_tasks: read_string_list_config("sbt.tasks", &properties, "SBT_TASKS", env)?,
        sbt_clean: read_boolean_config("sbt.clean", &properties, "SBT_CLEAN", env)?,
        sbt_available_at_launch: read_boolean_config(
            "sbt.available-at-launch",
            &properties,
            "SBT_AVAILABLE_AT_LAUNCH",
            env,
        )?,
        sbt_opts: read_sbt_opts(sbt_opts_file, env)?,
        sbt_version: get_declared_sbt_version(&app_dir)?,
    })
}

fn read_system_properties(app_dir: &Path) -> HashMap<String, String> {
    File::open(app_dir.join("system.properties"))
        .map(|file| java_properties::read(file).unwrap_or_default())
        .unwrap_or_default()
}

fn read_string_config(
    property_name: &str,
    system_properties: &HashMap<String, String>,
    environment_variable_name: &str,
    env: &Env,
) -> Result<Option<String>, ScalaBuildpackError> {
    if let Some(value) = system_properties.get(property_name) {
        return Ok(Some(value.clone()));
    }

    if let Some(value) = env.get(environment_variable_name) {
        let value = value.into_string().map_err(|e| {
            CouldNotConvertEnvironmentValueIntoString(environment_variable_name.to_string(), e)
        })?;
        return Ok(Some(value));
    }

    Ok(None)
}

fn read_boolean_config(
    property_name: &str,
    system_properties: &HashMap<String, String>,
    environment_variable_name: &str,
    env: &Env,
) -> Result<Option<bool>, ScalaBuildpackError> {
    if let Some(value) = system_properties.get(property_name) {
        return value
            .parse::<bool>()
            .map(Some)
            .map_err(|e| CouldNotParseBooleanFromProperty(property_name.to_string(), e));
    }

    if let Some(value) = env.get(environment_variable_name) {
        let value = value.into_string().map_err(|e| {
            CouldNotConvertEnvironmentValueIntoString(environment_variable_name.to_string(), e)
        })?;
        return value.parse::<bool>().map(Some).map_err(|e| {
            CouldNotParseBooleanFromEnvironment(environment_variable_name.to_string(), e)
        });
    }

    Ok(None)
}

fn read_string_list_config(
    property_name: &str,
    system_properties: &HashMap<String, String>,
    environment_variable_name: &str,
    env: &Env,
) -> Result<Option<Vec<String>>, ScalaBuildpackError> {
    if let Some(value) = system_properties.get(property_name) {
        return shell_words::split(value)
            .map(Some)
            .map_err(|e| CouldNotParseListConfigurationFromProperty(property_name.to_string(), e));
    }

    if let Some(value) = env.get(environment_variable_name) {
        let value = value.into_string().map_err(|e| {
            CouldNotConvertEnvironmentValueIntoString(environment_variable_name.to_string(), e)
        })?;
        return shell_words::split(&value).map(Some).map_err(|e| {
            CouldNotParseListConfigurationFromEnvironment(environment_variable_name.to_string(), e)
        });
    }

    Ok(None)
}

fn read_sbt_opts(
    opts_file: PathBuf,
    env: &Env,
) -> Result<Option<Vec<String>>, ScalaBuildpackError> {
    let mut sbt_opts: Vec<String> = vec![];
    let mut configured = false;

    if opts_file.exists() {
        let contents = read_to_string(opts_file).map_err(CouldNotReadSbtOptsFile)?;
        let mut opts =
            shell_words::split(&contents).map_err(CouldNotParseListConfigurationFromSbtOptsFile)?;
        sbt_opts.append(&mut opts);
        configured = true;
    }

    if let Some(value) = env.get("SBT_OPTS") {
        let value = value
            .into_string()
            .map_err(|e| CouldNotConvertEnvironmentValueIntoString("SBT_OPTS".to_string(), e))?;
        let mut opts = shell_words::split(&value).map_err(|e| {
            CouldNotParseListConfigurationFromEnvironment("SBT_OPTS".to_string(), e)
        })?;
        sbt_opts.append(&mut opts);
        configured = true;
    }

    if configured {
        Ok(Some(sbt_opts))
    } else {
        Ok(None)
    }
}

fn get_declared_sbt_version(app_dir: &Path) -> Result<Version, ScalaBuildpackError> {
    let build_properties_path = app_dir.join("project").join("build.properties");

    if !build_properties_path.exists() {
        return Err(MissingSbtBuildPropertiesFile);
    }

    let build_properties_file =
        File::open(build_properties_path).map_err(SbtPropertiesFileReadError)?;

    let properties =
        java_properties::read(build_properties_file).map_err(InvalidSbtPropertiesFile)?;

    let declared_version = properties.get("sbt.version").cloned().unwrap_or_default();
    if declared_version.is_empty() {
        return Err(MissingDeclaredSbtVersion);
    }

    // Note: while sbt didn't officially adopt semver until the 1.x version, all the published
    // versions listed in the repositories below do parse into the semver format:
    // - https://scala.jfrog.io/ui/native/ivy-releases/org.scala-tools.sbt/sbt-launch/
    // - https://scala.jfrog.io/ui/native/ivy-releases/org.scala-sbt/sbt-launch/
    // - https://repo1.maven.org/maven2/org/scala-sbt/sbt-launch/
    let version = Version::parse(&declared_version)
        .map_err(|error| SbtVersionNotInSemverFormat(declared_version, error))?;

    // this version range seemed odd to me but i think there's an upper-bound set to 0.13 because
    // the maven listing (https://repo1.maven.org/maven2/org/scala-sbt/sbt-launch/) contains
    // both a 0.99.2 and 0.99.4 release
    let version_0_required =
        VersionReq::parse(">=0.11, <=0.13").expect("Invalid version requirement");
    let version_1_required = VersionReq::parse(">=1, <2").expect("Invalid version requirement");
    let is_supported_version =
        version_0_required.matches(&version) || version_1_required.matches(&version);

    if !is_supported_version {
        return Err(UnsupportedSbtVersion(version.to_string()));
    }

    Ok(version)
}

#[cfg(test)]
mod create_build_config_tests {
    use crate::build_configuration::{
        create_build_config, Env, File, HashMap, MissingDeclaredSbtVersion,
        MissingSbtBuildPropertiesFile, UnsupportedSbtVersion, Version,
    };
    use crate::errors::ScalaBuildpackError;
    use std::ffi::{OsStr, OsString};
    use std::fs::{create_dir, write};
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
    fn create_build_config_raises_error_if_project_is_missing_the_sbt_build_properties_file() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        let error = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(error, MissingSbtBuildPropertiesFile);
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_version_property_is_missing_from_the_sbt_build_properties_file(
    ) {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        let sbt_project_path = app_dir.path().join("project");
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("build.properties"), "").unwrap();
        let error = create_build_config(app_dir.path().to_path_buf(), &env).unwrap_err();
        assert_err!(error, MissingDeclaredSbtVersion);
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_version_property_is_declared_with_empty_value() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        let sbt_project_path = app_dir.path().join("project");
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("build.properties"), b"sbt.version=").unwrap();
        let error = create_build_config(app_dir.path().to_path_buf(), &env).unwrap_err();
        assert_err!(error, MissingDeclaredSbtVersion);
    }

    #[test]
    fn create_build_config_with_valid_sbt_version_when_version_has_garbage_whitespace() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        let sbt_project_path = app_dir.path().join("project");
        create_dir(&sbt_project_path).unwrap();
        write(
            sbt_project_path.join("build.properties"),
            b"   sbt.version   =  1.8.2\n\n",
        )
        .unwrap();
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_version, Version::parse("1.8.2").unwrap());
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_version_outside_the_lower_bound_of_the_required_v0_range(
    ) {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "0.10.99");
        let error = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(error, UnsupportedSbtVersion(version) if version == "0.10.99");
    }

    #[test]
    fn create_build_config_with_sbt_version_within_the_lower_bound_of_the_required_v0_range() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "0.11.0");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_version, Version::parse("0.11.0").unwrap());
    }

    #[test]
    fn create_build_config_with_sbt_version_within_the_upper_bound_of_the_required_v0_range() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "0.13.99");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_version, Version::parse("0.13.99").unwrap());
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_version_outside_the_upper_bound_of_the_required_v0_range(
    ) {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "0.14.0");
        let error = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(error, UnsupportedSbtVersion(version) if version == "0.14.0");
    }

    #[test]
    fn create_build_config_with_sbt_version_within_the_lower_bound_of_the_required_v1_range() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.0.0");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_version, Version::parse("1.0.0").unwrap());
    }

    #[test]
    fn create_build_config_with_sbt_version_within_the_upper_bound_of_the_required_v1_range() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.99.99");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_version, Version::parse("1.99.99").unwrap());
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_version_outside_the_upper_bound_of_the_required_v1_range(
    ) {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "2.0.0");
        let error = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(error, UnsupportedSbtVersion(version) if version == "2.0.0");
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
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ScalaBuildpackError::CouldNotConvertEnvironmentValueIntoString(name, _) if name == "SBT_PROJECT");
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
        assert_err!(err, ScalaBuildpackError::CouldNotParseListConfigurationFromProperty(name, _) if name == "sbt.pre-tasks");
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_environment_variable_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1\" task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ScalaBuildpackError::CouldNotParseListConfigurationFromEnvironment(name, _) if name == "SBT_PRE_TASKS");
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_pre_tasks_environment_variable_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ScalaBuildpackError::CouldNotConvertEnvironmentValueIntoString(name, _) if name == "SBT_PRE_TASKS");
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
        assert_err!(err, ScalaBuildpackError::CouldNotParseBooleanFromProperty(name, _) if name == "sbt.clean");
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
        assert_err!(err, ScalaBuildpackError::CouldNotParseBooleanFromEnvironment(name, _) if name == "SBT_CLEAN");
    }

    #[test]
    fn create_build_config_when_sbt_clean_is_configured_from_env_and_value_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_CLEAN", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ScalaBuildpackError::CouldNotConvertEnvironmentValueIntoString(name, _) if name == "SBT_CLEAN");
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
        assert_err!(err, ScalaBuildpackError::CouldNotParseListConfigurationFromProperty(name, _) if name == "sbt.tasks");
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_environment_variable_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", OsString::from("task1\" task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ScalaBuildpackError::CouldNotParseListConfigurationFromEnvironment(name, _) if name == "SBT_TASKS");
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_tasks_environment_variable_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_TASKS", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ScalaBuildpackError::CouldNotConvertEnvironmentValueIntoString(name, _) if name == "SBT_TASKS");
    }

    #[test]
    fn create_build_config_when_sbt_opts_is_not_configured() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_opts, None);
    }

    #[test]
    fn create_build_config_when_sbt_opts_is_configured_from_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_OPTS", OsString::from("-J-Xfoo -J-Xbar"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        let expected_opts: Vec<String> = vec![String::from("-J-Xfoo"), String::from("-J-Xbar")];
        assert_eq!(config.sbt_opts, Some(expected_opts));
    }

    #[test]
    fn create_build_config_when_sbt_opts_is_configured_from_sbtopts_file() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        write(app_dir.path().join(".sbtopts"), "-J-Xzip -J-Xzap").unwrap();
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        let expected_opts: Vec<String> = vec![String::from("-J-Xzip"), String::from("-J-Xzap")];
        assert_eq!(config.sbt_opts, Some(expected_opts));
    }

    #[test]
    fn create_build_config_when_sbt_opts_is_configured_from_both_env_and_sbtopts_file() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_OPTS", OsString::from("-J-Xfoo -J-Xbar"));
        write(app_dir.path().join(".sbtopts"), "-J-Xzip -J-Xzap").unwrap();
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        let expected_opts: Vec<String> = vec![
            String::from("-J-Xzip"),
            String::from("-J-Xzap"),
            String::from("-J-Xfoo"),
            String::from("-J-Xbar"),
        ];
        assert_eq!(config.sbt_opts, Some(expected_opts));
    }

    #[test]
    fn create_build_config_raises_error_when_sbtopts_file_values_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        write(app_dir.path().join(".sbtopts"), "-J-Xzip\" -J-Xzap").unwrap();
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(
            err,
            ScalaBuildpackError::CouldNotParseListConfigurationFromSbtOptsFile(_)
        );
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_opts_environment_variable_cannot_be_split() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_OPTS", OsString::from("-J-Xfoo\" -J-Xbar"));
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ScalaBuildpackError::CouldNotParseListConfigurationFromEnvironment(name, _) if name == "SBT_OPTS");
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_opts_environment_variable_contains_non_unicode_bytes(
    ) {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_OPTS", invalid_unicode_os_string());
        set_sbt_version(&app_dir, "1.8.2");
        let err = create_build_config(app_dir.path(), &env).unwrap_err();
        assert_err!(err, ScalaBuildpackError::CouldNotConvertEnvironmentValueIntoString(name, _) if name == "SBT_OPTS");
    }
}
