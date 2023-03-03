use std::path::{Path, PathBuf};

pub(crate) fn sbt_project_path(app_dir: &Path) -> PathBuf {
    app_dir.join("project")
}

pub(crate) fn sbt_project_build_properties_path(app_dir: &Path) -> PathBuf {
    sbt_project_path(app_dir).join("build.properties")
}

pub(crate) fn system_properties_path(app_dir: &Path) -> PathBuf {
    app_dir.join("system.properties")
}
