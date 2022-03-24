use libcnb::Env;
use libherokubuildpack::DownloadError;
use std::env::temp_dir;
use std::path::{Path, PathBuf};

pub fn resolve_settings_xml_path<P: AsRef<Path>>(
    app_dir: P,
    env: &Env,
) -> Result<Option<PathBuf>, SettingsError> {
    handle_maven_settings_path_env_var(app_dir.as_ref(), env)
        .or_else(|| handle_maven_settings_url_env_var(env))
        .or_else(|| handle_implicit_settings_xml(app_dir.as_ref()).map(Ok))
        .transpose()
}

#[derive(Debug)]
pub enum SettingsError {
    InvalidMavenSettingsPath(PathBuf),
    DownloadError(String, DownloadError),
}

fn handle_maven_settings_path_env_var<P: AsRef<Path>>(
    app_dir: P,
    env: &Env,
) -> Option<Result<PathBuf, SettingsError>> {
    env.get("MAVEN_SETTINGS_PATH").map(|maven_settings_path| {
        let maven_settings_path = app_dir.as_ref().join(maven_settings_path);

        if maven_settings_path.exists() {
            Ok(maven_settings_path)
        } else {
            Err(SettingsError::InvalidMavenSettingsPath(maven_settings_path))
        }
    })
}

fn handle_maven_settings_url_env_var(env: &Env) -> Option<Result<PathBuf, SettingsError>> {
    env.get("MAVEN_SETTINGS_URL").map(|maven_settings_url| {
        let path = temp_dir().join(SETTINGS_XML_FILENAME);

        libherokubuildpack::download_file(maven_settings_url.to_string_lossy(), &path)
            .map_err(|error| {
                SettingsError::DownloadError(
                    maven_settings_url.to_string_lossy().to_string(),
                    error,
                )
            })
            .map(|_| path)
    })
}

fn handle_implicit_settings_xml<P: AsRef<Path>>(app_dir: P) -> Option<PathBuf> {
    Some(app_dir.as_ref().join(SETTINGS_XML_FILENAME)).filter(|path| path.exists())
}

const SETTINGS_XML_FILENAME: &str = "settings.xml";
