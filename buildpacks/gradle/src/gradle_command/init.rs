use buildpacks_jvm_shared::fs::list_directory_contents;
use std::path::{Path, PathBuf};

pub(crate) fn find_init_scripts(app_dir: &Path) -> Vec<PathBuf> {
    list_directory_contents(app_dir.join(".heroku/gradle/init.d"))
        .map(|paths| {
            paths
                .filter(|path| {
                    GRADLE_INIT_SCRIPT_SUFFIXES
                        .iter()
                        .any(|suffix| path.to_string_lossy().ends_with(suffix))
                })
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn gradle_init_script_args(init_script_paths: &[PathBuf]) -> Vec<String> {
    init_script_paths
        .iter()
        .flat_map(|init_script_path| {
            vec![
                String::from("--init-script"),
                init_script_path.to_string_lossy().into_owned(),
            ]
        })
        .collect()
}

// https://docs.gradle.org/8.5/userguide/init_scripts.html#sec:using_an_init_script
const GRADLE_INIT_SCRIPT_SUFFIXES: [&str; 2] = [".gradle", ".init.gradle.kts"];
