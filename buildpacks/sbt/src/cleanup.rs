use buildpacks_jvm_shared::{default_on_not_found, list_directory_contents};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

// the native package plugin produces binaries in the target/universal/stage directory which is not included
// in the list of directories to clean up at the end of the build since a Procfile may reference this
// location to provide the entry point for an application. wiping the directory before the application build
// kicks off will ensure that no leftover artifacts are being carried around between builds.
pub(crate) fn cleanup_any_existing_native_packager_directories(
    app_dir: &Path,
) -> std::io::Result<()> {
    default_on_not_found(fs::remove_dir_all(
        app_dir.join("target").join("universal").join("stage"),
    ))
}

pub(crate) fn cleanup_compilation_artifacts(app_dir: &Path) -> std::io::Result<()> {
    let target_dir = app_dir.join("target");

    let target_dir_files = list_directory_contents(&target_dir)?
        .filter(|path| match path.file_name().and_then(OsStr::to_str) {
            Some(file_name) => file_name.starts_with("scala-") || file_name == "streams",
            None => false,
        })
        .collect::<Vec<_>>();

    let resolution_cache_files = default_on_not_found(
        list_directory_contents(target_dir.join("resolution-cache")).map(|directory_contents| {
            directory_contents
                .filter(|path| match path.file_name().and_then(OsStr::to_str) {
                    Some(file_name) => {
                        !(file_name.ends_with("-compile.xml") || file_name == "reports")
                    }
                    None => true,
                })
                .collect::<Vec<_>>()
        }),
    )?;

    [target_dir_files, resolution_cache_files]
        .into_iter()
        .flatten()
        .map(|path| {
            if path.is_dir() {
                fs::remove_dir_all(path)
            } else {
                fs::remove_file(path)
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|_| ())
}
