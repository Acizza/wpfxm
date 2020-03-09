use crate::err::{self, Result};
use snafu::ResultExt;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn subdirectories<P>(dir: P) -> Result<Vec<DirEntry>>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    let entries = fs::read_dir(dir).context(err::FileIO { path: dir })?;

    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry.context(err::EntryIO { dir })?;
        let etype = entry.file_type().context(err::EntryIO { dir })?;

        if !etype.is_dir() {
            continue;
        }

        dirs.push(entry);
    }

    Ok(dirs)
}
