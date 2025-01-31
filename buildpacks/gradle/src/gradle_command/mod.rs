mod daemon;
mod dependency_report;
mod tasks;

pub(crate) use daemon::start as start_daemon;
pub(crate) use daemon::stop as stop_daemon;
pub(crate) use dependency_report::{dependency_report, GradleDependencyReport};
pub(crate) use tasks::tasks;

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
