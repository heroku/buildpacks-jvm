use sha2::digest::{FixedOutput, Output, Update};
use std::fs::DirEntry;
use std::io::Read;
use std::path::{Path, PathBuf};

pub(crate) fn digest<D>(mut input: impl Read) -> Result<Output<D>, std::io::Error>
where
    D: Default + Update + FixedOutput,
{
    let mut digest = D::default();

    let mut buffer = [0x00; 1024];
    loop {
        let bytes_read = input.read(&mut buffer)?;

        if bytes_read > 0 {
            digest.update(&buffer[..bytes_read]);
        } else {
            break;
        }
    }

    Ok(digest.finalize_fixed())
}

pub(crate) fn list_directory_contents<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<PathBuf>> {
    std::fs::read_dir(path.as_ref())
        .and_then(Iterator::collect::<std::io::Result<Vec<DirEntry>>>)
        .map(|dir_entries| dir_entries.iter().map(DirEntry::path).collect())
}

pub(crate) fn zip_longest<A, B>(a: A, b: B) -> ZipLongest<A, B>
where
    A: Iterator,
    B: Iterator,
{
    ZipLongest { a, b }
}

pub(crate) struct ZipLongest<A, B> {
    a: A,
    b: B,
}

impl<A, B> Iterator for ZipLongest<A, B>
where
    A: Iterator,
    B: Iterator,
{
    type Item = (Option<A::Item>, Option<B::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let a_item = self.a.next();
        let b_item = self.b.next();

        if a_item.is_none() && b_item.is_none() {
            None
        } else {
            Some((a_item, b_item))
        }
    }
}
