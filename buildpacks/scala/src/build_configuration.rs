use crate::errors::ScalaBuildpackError;
use crate::errors::ScalaBuildpackError::{
    InvalidSbtPropertiesFile, MissingDeclaredSbtVersion, MissingSbtBuildPropertiesFile,
    SbtPropertiesFileReadError, SbtVersionNotInSemverFormat, UnsupportedSbtVersion,
};
use crate::paths::{sbt_project_build_properties_path, system_properties_path};
use libcnb::Env;
use semver::{Version, VersionReq};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct BuildConfiguration {
    pub sbt_project: Option<OsString>,
    pub sbt_pre_tasks: Option<OsString>,
    pub sbt_clean: Option<bool>,
    pub sbt_opts: Option<OsString>,
    pub sbt_version: Version,
}

pub fn create_build_config<PathLike: Into<PathBuf>>(
    app_dir: PathLike,
    env: &Env,
) -> Result<BuildConfiguration, ScalaBuildpackError> {
    let app_dir = app_dir.into();
    let system_properties = read_system_properties(&app_dir);
    Ok(BuildConfiguration {
        sbt_project: system_properties.get("sbt.project").map(OsString::from),
        sbt_pre_tasks: system_properties
            .get("sbt.pre-tasks")
            .map(OsString::from)
            .or_else(|| env.get("SBT_PRE_TASKS")),
        sbt_clean: system_properties
            .get("sbt.clean")
            .map(OsString::from)
            .or_else(|| env.get("SBT_CLEAN"))
            .map(|v| v.to_string_lossy().parse().unwrap_or_default()),
        sbt_opts: env.get("SBT_OPTS"),
        sbt_version: get_declared_sbt_version(&app_dir)?,
    })
}

fn read_system_properties(app_dir: &Path) -> HashMap<String, String> {
    File::open(system_properties_path(app_dir))
        .map(|file| java_properties::read(file).unwrap_or_default())
        .unwrap_or_default()
}

fn get_declared_sbt_version(app_dir: &Path) -> Result<Version, ScalaBuildpackError> {
    let build_properties_path = sbt_project_build_properties_path(app_dir);

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

    let version = Version::parse(&declared_version)
        .map_err(|error| SbtVersionNotInSemverFormat(declared_version, error))?;

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
        create_build_config, system_properties_path, Env, File, HashMap, MissingDeclaredSbtVersion,
        MissingSbtBuildPropertiesFile, OsString, UnsupportedSbtVersion, Version,
    };
    use crate::paths::sbt_project_path;
    use std::fs::{create_dir, write};
    use std::io::BufWriter;
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
        let sbt_project_path = sbt_project_path(app_dir.path());
        create_dir(&sbt_project_path).unwrap();
        let contents = format!("sbt.version={version}");
        write(sbt_project_path.join("build.properties"), contents).unwrap();
    }

    fn set_system_properties(app_dir: &TempDir, properties: HashMap<&str, &str>) {
        let property_file = File::create(system_properties_path(app_dir.path())).unwrap();
        let writer = BufWriter::new(property_file);
        let properties = properties
            .into_iter()
            .map(|(key, val)| (key.to_string(), val.to_string()))
            .collect();
        java_properties::write(writer, &properties).unwrap();
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
        let sbt_project_path = sbt_project_path(app_dir.path());
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("build.properties"), "").unwrap();
        let error = create_build_config(app_dir.path().to_path_buf(), &env).unwrap_err();
        assert_err!(error, MissingDeclaredSbtVersion);
    }

    #[test]
    fn create_build_config_raises_error_when_sbt_version_property_is_declared_with_empty_value() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        let sbt_project_path = sbt_project_path(app_dir.path());
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("build.properties"), b"sbt.version=").unwrap();
        let error = create_build_config(app_dir.path().to_path_buf(), &env).unwrap_err();
        assert_err!(error, MissingDeclaredSbtVersion);
    }

    #[test]
    fn create_build_config_with_valid_sbt_version_when_version_has_garbage_whitespace() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        let sbt_project_path = sbt_project_path(app_dir.path());
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
    fn create_build_config_when_sbt_project_is_configured() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(
            &app_dir,
            HashMap::from([("sbt.project", "testProjectName")]),
        );
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_project, Some(OsString::from("testProjectName")));
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
        assert_eq!(config.sbt_pre_tasks, Some(OsString::from("task1 task2")));
    }

    #[test]
    fn create_build_config_when_sbt_pre_tasks_is_configured_from_env() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1 task2"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_pre_tasks, Some(OsString::from("task1 task2")));
    }

    #[test]
    fn create_build_config_prefers_system_property_over_env_for_sbt_pre_tasks() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_PRE_TASKS", OsString::from("task1 task2"));
        set_sbt_version(&app_dir, "1.8.2");
        set_system_properties(&app_dir, HashMap::from([("sbt.pre-tasks", "task3 task4")]));
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_pre_tasks, Some(OsString::from("task3 task4")));
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
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, Some(false));
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
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_clean, Some(false));
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
    fn create_build_config_when_sbt_opts_is_not_configured() {
        let app_dir = tempdir().unwrap();
        let env = Env::new();
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_opts, None);
    }

    #[test]
    fn create_build_config_when_sbt_opts_is_configured() {
        let app_dir = tempdir().unwrap();
        let mut env = Env::new();
        env.insert("SBT_OPTS", OsString::from("testValue"));
        set_sbt_version(&app_dir, "1.8.2");
        let config = create_build_config(app_dir.path(), &env).unwrap();
        assert_eq!(config.sbt_opts, Some(OsString::from("testValue")));
    }
}
