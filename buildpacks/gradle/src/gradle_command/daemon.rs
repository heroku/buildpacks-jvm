use crate::gradle_command::GradleCommandError;
use crate::GRADLE_TASK_NAME_HEROKU_START_DAEMON;
use buildpacks_jvm_shared::output::{self, CmdError};
use libcnb::Env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

pub(crate) struct GradleDaemon {
    name: String,
    file: NamedTempFile,
    executable_path: PathBuf,
}

impl GradleDaemon {
    pub(crate) fn stop(self, gradle_env: &Env) -> Result<(), GradleCommandError<()>> {
        let file = self.file.reopen().map_err(GradleCommandError::Io)?;
        let _ = Command::new(self.executable_path)
            .args(["-q", "--stop"])
            .envs(gradle_env)
            .stdout(Stdio::from(
                file.try_clone().map_err(GradleCommandError::Io)?,
            ))
            .stderr(Stdio::from(file))
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(GradleCommandError::Io)?;

        Ok(())
    }
}

pub(crate) fn start(
    gradle_wrapper_executable_path: &Path,
    gradle_env: &Env,
) -> Result<GradleDaemon, GradleCommandError<()>> {
    let daemon = GradleDaemon {
        name: GRADLE_TASK_NAME_HEROKU_START_DAEMON.to_string(),
        executable_path: gradle_wrapper_executable_path.to_path_buf(),
        file: tempfile::NamedTempFile::new().map_err(GradleCommandError::Io)?,
    };
    std::fs::write(&daemon.file, "").map_err(GradleCommandError::Io)?;
    let mut command = Command::new(&daemon.executable_path);
    command
        .args([
            // Fixes an issue when running under Apple Rosetta emulation
            "-Djdk.lang.Process.launchMechanism=vfork",
            "--daemon",
            &daemon.name,
        ])
        .envs(gradle_env);
    let file = daemon.file.reopen().map_err(GradleCommandError::Io)?;
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

    Ok(daemon)
}
