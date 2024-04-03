use crate::{app_dependency_list_path, ProcessBuilder};
use buildpacks_jvm_shared::fs::list_directory_contents;
use libcnb::data::launch::Process;
use libcnb::data::process_type;
use std::path::Path;

#[derive(Copy, Clone)]
pub(crate) enum Framework {
    SpringBoot,
    WildflySwarm,
}

pub(crate) fn detect_framework<P: AsRef<Path>>(
    app_dir: P,
) -> Result<Option<Framework>, DetectFrameworkError> {
    let dependency_list_string =
        std::fs::read_to_string(app_dependency_list_path(app_dir.as_ref()))
            .map_err(DetectFrameworkError::IoError)?;

    let spring_boot_regex = regex::Regex::new("org.springframework.boot:spring-boot")
        .map_err(DetectFrameworkError::RegexError)?;

    let wildfly_swarm_regex =
        regex::Regex::new("org.wildfly.swarm").map_err(DetectFrameworkError::RegexError)?;

    let framework = [
        (spring_boot_regex, Framework::SpringBoot),
        (wildfly_swarm_regex, Framework::WildflySwarm),
    ]
    .iter()
    .find_map(|(regex, framework)| {
        regex
            .is_match(&dependency_list_string)
            .then_some(*framework)
    });

    Ok(framework)
}

pub(crate) fn default_app_process<P: AsRef<Path>>(
    app_dir: P,
) -> Result<Option<Process>, DefaultAppProcessError> {
    let framework =
        detect_framework(app_dir.as_ref()).map_err(DefaultAppProcessError::DetectFrameworkError)?;

    let main_jar_file_path = list_directory_contents(app_dir.as_ref().join("target"))
        .map(|mut paths| {
            paths.find(|path| {
                #[allow(clippy::case_sensitive_file_extension_comparisons)]
                path.file_name()
                    .map(|file_name| file_name.to_string_lossy().to_string())
                    .filter(|file_name| {
                        file_name.ends_with(".jar")
                            && !file_name.ends_with("-sources.jar")
                            && !file_name.ends_with("-javadoc.jar")
                    })
                    .is_some()
            })
        })
        .map_err(DefaultAppProcessError::IoError)?;

    let process = match (framework, main_jar_file_path) {
        (Some(Framework::SpringBoot), Some(main_jar_file_path)) => Some(
            ProcessBuilder::new(
                process_type!("web"),
                [
                    "bash",
                    "-c",
                    &format!(
                        "java -Dserver.port=$PORT $JAVA_OPTS -jar {}",
                        main_jar_file_path.to_string_lossy()
                    ),
                ],
            )
            .default(true)
            .build(),
        ),
        (Some(Framework::WildflySwarm), Some(main_jar_file_path)) => Some(
            ProcessBuilder::new(
                process_type!("web"),
                [
                    "bash",
                    "-c",
                    &format!(
                        "java -Dswarm.http.port=$PORT $JAVA_OPTS -jar {}",
                        main_jar_file_path.to_string_lossy()
                    ),
                ],
            )
            .default(true)
            .build(),
        ),
        _ => None,
    };

    Ok(process)
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum DefaultAppProcessError {
    DetectFrameworkError(DetectFrameworkError),
    IoError(std::io::Error),
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum DetectFrameworkError {
    IoError(std::io::Error),
    RegexError(regex::Error),
}
