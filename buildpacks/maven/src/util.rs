use buildpacks_jvm_shared::fs::list_directory_contents;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub(crate) fn move_directory_contents<P: AsRef<Path>, Q: AsRef<Path>>(
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

pub(crate) fn run_command<E, F: FnOnce(std::io::Error) -> E, F2: FnOnce(ExitStatus) -> E>(
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
