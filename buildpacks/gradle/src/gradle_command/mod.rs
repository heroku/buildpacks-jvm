mod daemon;
mod dependency_report;
mod tasks;

use buildpacks_jvm_shared::output::CmdError;
pub(crate) use daemon::start as start_daemon;
pub(crate) use dependency_report::{dependency_report, GradleDependencyReport};
use fun_run::CommandWithName;
pub(crate) use tasks::tasks;

use std::process::Command;

#[derive(Debug)]
pub(crate) enum GradleCommandError<P> {
    Io(std::io::Error),
    FailedCommand(CmdError),
    Parse(P),
}

impl<P> GradleCommandError<P> {
    pub(crate) fn map_parse_error<T, F>(self, f: F) -> GradleCommandError<T>
    where
        F: Fn(P) -> T,
    {
        match self {
            GradleCommandError::FailedCommand(e) => GradleCommandError::FailedCommand(e),
            GradleCommandError::Parse(p) => GradleCommandError::Parse(f(p)),
            GradleCommandError::Io(io_error) => GradleCommandError::Io(io_error),
        }
    }
}

fn run_gradle_command<T, F, P>(command: &mut Command, parser: F) -> Result<T, GradleCommandError<P>>
where
    F: FnOnce(&str, &str) -> Result<T, P>,
{
    let output = command
        .named_output()
        .map_err(GradleCommandError::FailedCommand)?;

    parser(&output.stdout_lossy(), &output.stderr_lossy()).map_err(GradleCommandError::Parse)
}
