use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn build_dir<P>(base: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let dir = base.as_ref().join(env!("CARGO_PKG_NAME"));

    if !dir.exists() {
        fs::create_dir_all(&dir).ok()?;
    }

    Some(dir)
}

pub fn get_config_dir() -> Option<PathBuf> {
    build_dir(dirs::config_dir()?)
}

pub fn get_data_dir() -> Option<PathBuf> {
    build_dir(dirs::data_local_dir()?)
}
