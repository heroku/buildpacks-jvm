use crate::{util, ProcessBuilder};
use libcnb::data::launch::Process;
use libcnb::data::process_type;
use std::path::Path;

#[derive(Copy, Clone)]
pub enum Framework {
    SpringBoot,
    WildflySwarm,
}

pub fn detect_framework<P: AsRef<Path>>(
    app_dir: P,
) -> Result<Option<Framework>, DetectFrameworkError> {
    let dependency_list_string =
        std::fs::read_to_string(app_dir.as_ref().join("target/mvn-dependency-list.log"))
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
    .find_map(|(regex, framework)| regex.is_match(&dependency_list_string).then(|| *framework));

    Ok(framework)
}

pub fn default_app_process<P: AsRef<Path>>(app_dir: P) -> Option<Process> {
    detect_framework(app_dir.as_ref())
        .unwrap()
        .and_then(|framework| {
            util::list_directory_contents(app_dir.as_ref().join("target"))
                .unwrap()
                .iter()
                .find(|path| {
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
                .map(|main_jar_file_path| match framework {
                    Framework::SpringBoot => {
                        format!(
                            "java -Dserver.port=$PORT $JAVA_OPTS -jar {}",
                            main_jar_file_path.to_string_lossy()
                        )
                    }
                    Framework::WildflySwarm => {
                        format!(
                            "java -Dsswarm.http.port=$PORT $JAVA_OPTS -jar {}",
                            main_jar_file_path.to_string_lossy()
                        )
                    }
                })
                .map(|command| {
                    // TODO: ARGS INSTEAD?
                    ProcessBuilder::new(process_type!("web"), command)
                        .default(true)
                        .build()
                })
        })
}

#[derive(Debug)]
pub enum DetectFrameworkError {
    IoError(std::io::Error),
    RegexError(regex::Error),
}
