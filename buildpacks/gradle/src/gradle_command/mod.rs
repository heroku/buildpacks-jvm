mod daemon;
mod dependency_report;
mod tasks;

pub(crate) use daemon::start as start_daemon;
pub(crate) use dependency_report::{dependency_report, GradleDependencyReport};
pub(crate) use tasks::tasks;

use std::process::Command;

#[derive(Debug)]
pub(crate) enum GradleCommandError<P> {
    Io(std::io::Error),
    UnexpectedExitStatus {
        status: std::process::ExitStatus,
        stdout: String,
        stderr: String,
    },
    Parse(P),
}

impl<P> GradleCommandError<P> {
    pub(crate) fn map_parse_error<T, F>(self, f: F) -> GradleCommandError<T>
    where
        F: Fn(P) -> T,
    {
        match self {
            GradleCommandError::Parse(p) => GradleCommandError::Parse(f(p)),
            GradleCommandError::Io(io_error) => GradleCommandError::Io(io_error),
            GradleCommandError::UnexpectedExitStatus {
                status,
                stdout,
                stderr,
            } => GradleCommandError::UnexpectedExitStatus {
                status,
                stdout,
                stderr,
            },
        }
    }
}

fn run_gradle_command<T, F, P>(command: &mut Command, parser: F) -> Result<T, GradleCommandError<P>>
where
    F: FnOnce(&str, &str) -> Result<T, P>,
{
    let output = command.output().map_err(GradleCommandError::Io)?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    if output.status.success() {
        parser(&stdout, &stderr).map_err(GradleCommandError::Parse)
    } else {
        Err(GradleCommandError::UnexpectedExitStatus {
            status: output.status,
            stdout,
            stderr,
        })
    }
}
