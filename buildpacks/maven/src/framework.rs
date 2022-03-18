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

#[derive(Debug)]
pub enum DetectFrameworkError {
    IoError(std::io::Error),
    RegexError(regex::Error),
}
