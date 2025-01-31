use crate::gradle_command::GradleCommandError;
use crate::GRADLE_TASK_NAME_HEROKU_START_DAEMON;
use buildpacks_jvm_shared::output;
use libcnb::Env;
use std::path::Path;
use std::process::Command;

pub(crate) fn start(
    gradle_wrapper_executable_path: &Path,
    gradle_env: &Env,
) -> Result<(), GradleCommandError<()>> {
    let mut command = Command::new(gradle_wrapper_executable_path);
    command
        .args([
            // Fixes an issue when running under Apple Rosetta emulation
            "-Djdk.lang.Process.launchMechanism=vfork",
            "--daemon",
            GRADLE_TASK_NAME_HEROKU_START_DAEMON,
        ])
        .envs(gradle_env);

    output::run_command(command, false, GradleCommandError::Io, |output| {
        GradleCommandError::UnexpectedExitStatus {
            status: output.status,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    })
    .map(|_| ())
}

pub(crate) fn stop(
    gradle_wrapper_executable_path: &Path,
    gradle_env: &Env,
) -> Result<(), GradleCommandError<()>> {
    let mut command = Command::new(gradle_wrapper_executable_path);
    command.args(["-q", "--stop"]).envs(gradle_env);

    output::run_command(command, true, GradleCommandError::Io, |output| {
        GradleCommandError::UnexpectedExitStatus {
            status: output.status,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    })
    .map(|_| ())
}
