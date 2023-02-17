use crate::paths::{sbt_project_build_properties_path, sbt_project_path};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub fn detect_sbt(app_dir: &Path) -> bool {
    has_sbt_file(app_dir)
        || has_project_scala_file(app_dir)
        || has_hidden_sbt_directory(app_dir)
        || has_build_properties_file(app_dir)
}

fn has_sbt_file(app_dir: &Path) -> bool {
    find_in_directory(app_dir, file_ends_in("sbt"))
}

fn has_project_scala_file(app_dir: &Path) -> bool {
    find_in_directory(&sbt_project_path(app_dir), file_ends_in("scala"))
}

fn has_hidden_sbt_directory(app_dir: &Path) -> bool {
    find_in_directory(&app_dir.join(".sbt"), file_ends_in("scala"))
}

fn has_build_properties_file(app_dir: &Path) -> bool {
    sbt_project_build_properties_path(app_dir).exists()
}

fn find_in_directory(directory: &Path, predicate: impl Fn(PathBuf) -> bool) -> bool {
    directory
        .read_dir()
        .map(|entries| {
            entries
                .into_iter()
                .any(|entry| entry.map(|f| predicate(f.path())).unwrap_or(false))
        })
        .unwrap_or(false)
}

fn file_ends_in<Ext: Into<OsString>>(ext: Ext) -> impl Fn(PathBuf) -> bool {
    let ext = ext.into();
    move |path: PathBuf| path.extension() == Some(ext.as_os_str())
}

#[cfg(test)]
mod detect_sbt_tests {
    use crate::detection::{detect_sbt, sbt_project_path};
    use std::fs::{create_dir, write};
    use tempfile::tempdir;

    #[test]
    fn detect_sbt_fails_when_no_sbt_files_in_application_directory() {
        let app_dir = tempdir().unwrap();
        assert!(!detect_sbt(app_dir.path()));
    }

    #[test]
    fn detect_sbt_passes_when_an_sbt_file_is_found_in_application_directory() {
        let app_dir = tempdir().unwrap();
        write(app_dir.path().join("build.sbt"), "").unwrap();
        assert!(detect_sbt(app_dir.path()));
    }

    #[test]
    fn detect_sbt_passes_when_a_scala_file_is_found_in_the_sbt_project_directory() {
        let app_dir = tempdir().unwrap();
        let sbt_project_path = sbt_project_path(app_dir.path());
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("some-file.scala"), "").unwrap();
        assert!(detect_sbt(app_dir.path()));
    }

    #[test]
    fn detect_sbt_passes_when_hidden_sbt_directory_is_found_in_application_directory() {
        let app_dir = tempdir().unwrap();
        let dot_sbt = app_dir.path().join(".sbt");
        create_dir(&dot_sbt).unwrap();
        write(dot_sbt.join("some-file.scala"), "").unwrap();
        assert!(detect_sbt(app_dir.path()));
    }

    #[test]
    fn detect_sbt_passes_when_build_properties_file_is_found_in_the_sbt_project_directory() {
        let app_dir = tempdir().unwrap();
        let sbt_project_path = sbt_project_path(app_dir.path());
        create_dir(&sbt_project_path).unwrap();
        write(sbt_project_path.join("build.properties"), "").unwrap();
        assert!(detect_sbt(app_dir.path()));
    }
}
