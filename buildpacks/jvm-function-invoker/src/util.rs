pub mod logger;

use sha2::Digest;
use std::{fs, io};

pub fn download(uri: impl AsRef<str>, dst: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let response = reqwest::blocking::get(uri.as_ref())?;
    let mut content = io::Cursor::new(response.bytes()?);
    let mut file = fs::File::create(dst.as_ref())?;
    io::copy(&mut content, &mut file)?;

    Ok(())
}

pub fn sha256(data: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(data))
}
