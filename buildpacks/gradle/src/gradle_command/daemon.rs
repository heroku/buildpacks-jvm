use crate::gradle_command::GradleCommandError;
use crate::GRADLE_TASK_NAME_HEROKU_START_DAEMON;
use libcnb::Env;
use libherokubuildpack::command::CommandExt;
use std::io::{stderr, stdout};
use std::path::Path;
use std::process::Command;

pub(crate) fn start(
    gradle_wrapper_executable_path: &Path,
    gradle_env: &Env,
) -> Result<(), GradleCommandError<()>> {
    let output = Command::new(gradle_wrapper_executable_path)
        .args([
            // Fixes an issue when when running under Apple Rosetta emulation
            "-Djdk.lang.Process.launchMechanism=vfork",
            "--daemon",
            GRADLE_TASK_NAME_HEROKU_START_DAEMON,
        ])
        .envs(gradle_env)
        .output_and_write_streams(stdout(), stderr())
        .map_err(GradleCommandError::Io)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GradleCommandError::UnexpectedExitStatus {
            status: output.status,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

pub(crate) fn stop(
    gradle_wrapper_executable_path: &Path,
    gradle_env: &Env,
) -> Result<(), GradleCommandError<()>> {
    Command::new(gradle_wrapper_executable_path)
        .args(["-q", "--stop"])
        .envs(gradle_env)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(GradleCommandError::Io)?;

    Ok(())
}
