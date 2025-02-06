use crate::gradle_command::GradleCommandError;
use libcnb::Env;
use std::path::Path;
use std::process::Command;

pub(crate) fn stop(
    gradle_wrapper_executable_path: &Path,
    gradle_env: &Env,
) -> Result<(), GradleCommandError<()>> {
    Command::new(gradle_wrapper_executable_path)
        .args(["-q", "--stop"])
        .envs(gradle_env)
        .output()
        .map_err(GradleCommandError::Io)?;

    Ok(())
}
