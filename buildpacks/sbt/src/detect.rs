use buildpacks_jvm_shared::{default_on_not_found, list_directory_contents};
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

pub(crate) fn is_sbt_project_directory(root_dir: &Path) -> io::Result<bool> {
    let root_dir_result = list_directory_contents(root_dir)?.any(has_sbt_extension);

    let project_dir_result = default_on_not_found(
        list_directory_contents(root_dir.join("project"))
            .map(|mut entries| entries.any(has_scala_extension)),
    )?;

    let dot_sbt_dir_result = default_on_not_found(
        list_directory_contents(root_dir.join(".sbt"))
            .map(|mut entries| entries.any(has_scala_extension)),
    )?;

    let build_properties_result = root_dir
        .join("project")
        .join("build.properties")
        .try_exists()?;

    Ok(root_dir_result || project_dir_result || dot_sbt_dir_result || build_properties_result)
}

#[allow(clippy::needless_pass_by_value)]
fn has_scala_extension(path: PathBuf) -> bool {
    path.extension() == Some(OsStr::new("scala"))
}

#[allow(clippy::needless_pass_by_value)]
fn has_sbt_extension(path: PathBuf) -> bool {
    path.extension() == Some(OsStr::new("sbt"))
}

#[cfg(test)]
mod tests {
    use super::is_sbt_project_directory;
    use std::fs::{create_dir, write};
    use tempfile::tempdir;

    #[test]
    fn is_sbt_project_directory_fails_when_no_sbt_files_in_application_directory() {
        let app_dir = tempdir().unwrap();
        assert!(!is_sbt_project_directory(app_dir.path()).unwrap());
    }

    #[test]
    fn is_sbt_project_directory_passes_when_an_sbt_file_is_found_in_application_directory() {
        let app_dir = tempdir().unwrap();
        write(app_dir.path().join("build.sbt"), "").unwrap();
        assert!(is_sbt_project_directory(app_dir.path()).unwrap());
    }

    #[test]
    fn is_sbt_project_directory_passes_when_a_scala_file_is_found_in_the_sbt_project_directory() {
        let app_dir = tempdir().unwrap();
        let sbt_project_path = app_dir.path().join("project");
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("some-file.scala"), "").unwrap();
        assert!(is_sbt_project_directory(app_dir.path()).unwrap());
    }

    #[test]
    fn is_sbt_project_directory_passes_when_hidden_sbt_directory_is_found_in_application_directory()
    {
        let app_dir = tempdir().unwrap();
        let dot_sbt = app_dir.path().join(".sbt");
        create_dir(&dot_sbt).unwrap();
        write(dot_sbt.join("some-file.scala"), "").unwrap();
        assert!(is_sbt_project_directory(app_dir.path()).unwrap());
    }

    #[test]
    fn is_sbt_project_directory_passes_when_build_properties_file_is_found_in_the_sbt_project_directory(
    ) {
        let app_dir = tempdir().unwrap();
        let sbt_project_path = app_dir.path().join("project");
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("build.properties"), "").unwrap();
        assert!(is_sbt_project_directory(app_dir.path()).unwrap());
    }
}
