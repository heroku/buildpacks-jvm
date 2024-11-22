use flate2::read::GzDecoder;
use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Archive;

pub(crate) fn extract_tarball(
    file: &mut File,
    destination: &Path,
    strip_components: usize,
) -> Result<(), std::io::Error> {
    let mut archive = Archive::new(GzDecoder::new(file));

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;

        let entry_destination = path
            .components()
            .skip(strip_components)
            .fold(PathBuf::from(destination), |acc, item| acc.join(item));

        if let Some(entry_destination_parent) = entry_destination.parent() {
            std::fs::create_dir_all(entry_destination_parent)?;
        }

        entry.unpack(entry_destination)?;
    }

    Ok(())
}
