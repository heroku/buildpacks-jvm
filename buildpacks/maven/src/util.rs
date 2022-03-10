use std::fs::DirEntry;
use std::path::Path;

pub fn move_directory_contents<P: AsRef<Path>, Q: AsRef<Path>>(
    from: P,
    to: Q,
) -> std::io::Result<()> {
    let dir_entries = std::fs::read_dir(from.as_ref())
        .and_then(Iterator::collect::<std::io::Result<Vec<DirEntry>>>)?;

    for dir_entry in dir_entries {
        std::fs::rename(
            dir_entry.path(),
            to.as_ref()
                .join(dir_entry.path().components().last().unwrap()),
        )?;
    }

    Ok(())
}
