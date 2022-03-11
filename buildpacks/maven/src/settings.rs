use crate::MavenBuildpack;
use libcnb::build::BuildContext;
use libcnb::Platform;
use libherokubuildpack::DownloadError;
use std::env::temp_dir;
use std::path::PathBuf;

pub fn handle_maven_settings_path_env_var(
    context: &BuildContext<MavenBuildpack>,
) -> Option<Result<PathBuf, SettingsError>> {
    context
        .platform
        .env()
        .get("MAVEN_SETTINGS_PATH")
        .map(|maven_settings_path| {
            let maven_settings_path = context.app_dir.join(maven_settings_path);

            if maven_settings_path.exists() {
                Ok(maven_settings_path)
            } else {
                Err(SettingsError::InvalidMavenSettingsPath(maven_settings_path))
            }
        })
}

pub fn handle_maven_settings_url_env_var(
    context: &BuildContext<MavenBuildpack>,
) -> Option<Result<PathBuf, SettingsError>> {
    context
        .platform
        .env()
        .get("MAVEN_SETTINGS_URL")
        .map(|maven_settings_url| {
            let path = temp_dir().join("settings.xml");

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

pub fn handle_implicit_settings_xml(context: &BuildContext<MavenBuildpack>) -> Option<PathBuf> {
    Some(context.app_dir.join("settings.xml")).filter(|path| path.exists())
}

pub fn resolve_settings_xml_path(
    context: &BuildContext<MavenBuildpack>,
) -> Result<Option<PathBuf>, SettingsError> {
    handle_maven_settings_path_env_var(context)
        .or_else(|| handle_maven_settings_url_env_var(context))
        .or_else(|| handle_implicit_settings_xml(context).map(Ok))
        .transpose()
}

#[derive(Debug)]
pub enum SettingsError {
    InvalidMavenSettingsPath(PathBuf),
    DownloadError(String, DownloadError),
}
