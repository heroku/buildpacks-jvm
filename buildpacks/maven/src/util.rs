use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

pub fn move_directory_contents<P: AsRef<Path>, Q: AsRef<Path>>(
    from: P,
    to: Q,
) -> std::io::Result<()> {
    let dir_entries = list_directory_contents(from.as_ref())?;

    for dir_entry in dir_entries {
        std::fs::rename(
            &dir_entry,
            to.as_ref().join(dir_entry.components().last().unwrap()),
        )?;
    }

    Ok(())
}

pub fn list_directory_contents<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<PathBuf>> {
    std::fs::read_dir(path.as_ref())
        .and_then(Iterator::collect::<std::io::Result<Vec<DirEntry>>>)
        .map(|dir_entries| {
            dir_entries
                .iter()
                .map(|dir_entry| dir_entry.path())
                .collect()
        })
}

pub fn run_command<E, F: FnOnce(std::io::Error) -> E, F2: FnOnce(ExitStatus) -> E>(
    command: &mut Command,
    io_error_fn: F,
    exit_status_fn: F2,
) -> Result<ExitStatus, E> {
    command
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(io_error_fn)
        .and_then(|exit_status| {
            if exit_status.success() {
                Ok(exit_status)
            } else {
                Err(exit_status_fn(exit_status))
            }
        })
}
