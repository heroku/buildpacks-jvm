use std::path::Path;

pub(crate) fn is_gradle_project_directory(root_dir: &Path) -> std::io::Result<bool> {
    // We look for these Gradle specific files and not 'gradlew' directly. This allows us to
    // fail with an error message explaining that 'gradlew' is required for Gradle projects.
    // If we just fail detect on a missing Gradle Wrapper, we lose the opportunity to display
    // such a message, worsening DX.
    [
        "build.gradle",
        "settings.gradle",
        "build.gradle.kts",
        "settings.gradle.kts",
    ]
    .into_iter()
    .map(|file_name| root_dir.join(file_name).try_exists())
    .collect::<Result<Vec<_>, _>>()
    .map(|results| results.contains(&true))
}
