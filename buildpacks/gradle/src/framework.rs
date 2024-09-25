use crate::gradle_command::GradleDependencyReport;
use buildpacks_jvm_shared::fs::list_directory_contents;
use libcnb::data::launch::{Process, ProcessBuilder};
use libcnb::data::process_type;
use std::path::Path;

pub(crate) fn detect_framework(dependency_report: &GradleDependencyReport) -> Option<Framework> {
    DEPENDENCY_TO_FRAMEWORK_MAPPINGS.into_iter().find_map(
        |(configuration, group_id, artifact_id, framework)| {
            dependency_report
                .contains_dependency(configuration, group_id, artifact_id)
                .then_some(framework)
        },
    )
}

#[allow(clippy::case_sensitive_file_extension_comparisons)]
pub(crate) fn default_app_process<P: AsRef<Path>>(
    dependency_report: &GradleDependencyReport,
    app_dir: P,
) -> Result<Option<Process>, std::io::Error> {
    let jar_path = match detect_framework(dependency_report) {
        Some(Framework::SpringBoot | Framework::Micronaut) => {
            list_directory_contents(app_dir.as_ref().join("build/libs"))?.find(|path| {
                path.file_name()
                    .map(|file_name| file_name.to_string_lossy().to_string())
                    .is_some_and(|file_name| {
                        file_name.ends_with(".jar")
                        && !file_name.ends_with("-plain.jar") // Spring Boot JAR without dependencies
                        && !file_name.ends_with("-sources.jar")
                        && !file_name.ends_with("-javadoc.jar")
                    })
            })
        }
        Some(Framework::Quarkus) => {
            let quarkus_run_jar = app_dir.as_ref().join("build/quarkus-app/quarkus-run.jar");
            quarkus_run_jar.is_file().then_some(quarkus_run_jar)
        }
        _ => None,
    };

    let process = jar_path.map(|jar_path| {
        ProcessBuilder::new(
            process_type!("web"),
            ["java", "-jar", &jar_path.to_string_lossy()],
        )
        .default(true)
        .build()
    });

    Ok(process)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Framework {
    Ratpack,
    SpringBoot,
    Micronaut,
    Quarkus,
}

const DEPENDENCY_TO_FRAMEWORK_MAPPINGS: [(&str, &str, &str, Framework); 4] = [
    (
        "runtimeClasspath",
        "org.springframework.boot",
        "spring-boot",
        Framework::SpringBoot,
    ),
    (
        "runtimeClasspath",
        "io.ratpack",
        "ratpack-core",
        Framework::Ratpack,
    ),
    (
        "runtimeClasspath",
        "io.micronaut",
        "micronaut-core",
        Framework::Micronaut,
    ),
    (
        "quarkusProdRuntimeClasspathConfigurationDeployment",
        "io.quarkus",
        "quarkus-core",
        Framework::Quarkus,
    ),
];
