use anyhow::{anyhow, Context, Result};
use std::borrow::Cow;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

pub fn subdirectories<P>(dir: P) -> Result<Vec<DirEntry>>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    let entries = fs::read_dir(dir).with_context(|| anyhow!("failed to read {}", dir.display()))?;

    let mut dirs = Vec::new();

    for entry in entries {
        let entry =
            entry.with_context(|| anyhow!("failed to read dir entry in {}", dir.display()))?;

        let etype = entry
            .file_type()
            .with_context(|| anyhow!("failed to read dir entry in {}", dir.display()))?;

        if !etype.is_dir() {
            continue;
        }

        dirs.push(entry);
    }

    Ok(dirs)
}

pub fn strip_base_path<'a, B, P>(base: B, path: P) -> PathBuf
where
    B: AsRef<Path>,
    P: Into<Cow<'a, Path>>,
{
    let path = path.into();

    match path.strip_prefix(base) {
        Ok(path) => path.into(),
        Err(_) => path.into_owned(),
    }
}
