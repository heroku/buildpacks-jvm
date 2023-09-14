use crate::gradle_command::GradleDependencyReport;

pub(crate) fn detect_framework(dependency_report: &GradleDependencyReport) -> Option<Framework> {
    DEPENDENCY_TO_FRAMEWORK_MAPPINGS
        .into_iter()
        .find_map(|(group_id, artifact_id, framework)| {
            dependency_report
                .contains_dependency("runtimeClasspath", group_id, artifact_id)
                .then_some(framework)
        })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Framework {
    Ratpack,
    SpringBoot,
}

const DEPENDENCY_TO_FRAMEWORK_MAPPINGS: [(&str, &str, Framework); 2] = [
    ("io.ratpack", "ratpack-core", Framework::Ratpack),
    (
        "org.springframework.boot",
        "spring-boot",
        Framework::SpringBoot,
    ),
];
