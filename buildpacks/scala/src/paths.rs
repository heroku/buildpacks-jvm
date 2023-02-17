use std::path::{Path, PathBuf};

pub fn sbt_project_path(app_dir: &Path) -> PathBuf {
    app_dir.join("project")
}

pub fn sbt_project_build_properties_path(app_dir: &Path) -> PathBuf {
    sbt_project_path(app_dir).join("build.properties")
}

pub fn system_properties_path(app_dir: &Path) -> PathBuf {
    app_dir.join("system.properties")
}
