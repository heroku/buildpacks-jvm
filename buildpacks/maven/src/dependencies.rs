use std::path::Path;

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

    if spring_boot_regex.is_match(&dependency_list_string) {
        Ok(Some(Framework::SpringBoot))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub enum DetectFrameworkError {
    IoError(std::io::Error),
    RegexError(regex::Error),
}
