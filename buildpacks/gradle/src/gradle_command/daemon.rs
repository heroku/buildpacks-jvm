use crate::gradle_command::GradleCommandError;
use crate::GRADLE_TASK_NAME_HEROKU_START_DAEMON;
use buildpacks_jvm_shared::output::{self, CmdError};
use libcnb::Env;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

pub(crate) struct GradleDaemonLog {
    file: NamedTempFile,
}

pub(crate) fn start(
    gradle_wrapper_executable_path: &Path,
    gradle_env: &Env,
) -> Result<GradleDaemonLog, GradleCommandError<()>> {
    let log = GradleDaemonLog {
        file: tempfile::NamedTempFile::new().map_err(GradleCommandError::Io)?,
    };
    std::fs::write(&log.file, "").map_err(GradleCommandError::Io)?;
    let mut command = Command::new(gradle_wrapper_executable_path);
    command
        .args([
            // Fixes an issue when running under Apple Rosetta emulation
            "-Djdk.lang.Process.launchMechanism=vfork",
            "--daemon",
            GRADLE_TASK_NAME_HEROKU_START_DAEMON,
        ])
        .envs(gradle_env);
    let file = log.file.reopen().map_err(GradleCommandError::Io)?;

    command.stdout(Stdio::from(
        file.try_clone().map_err(GradleCommandError::Io)?,
    ));
    command.stderr(Stdio::from(file));

    let _ = output::run_command(command, true).map_err(|error| match error {
        CmdError::SystemError(_, error) => GradleCommandError::Io(error),
        CmdError::NonZeroExitNotStreamed(named_output)
        | CmdError::NonZeroExitAlreadyStreamed(named_output) => {
            GradleCommandError::UnexpectedExitStatus {
                status: *named_output.status(),
                stdout: named_output.stdout_lossy(),
                stderr: named_output.stderr_lossy(),
            }
        }
    })?;
    Ok(log)
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
