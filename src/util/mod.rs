pub mod dir;

use std::path::{Path, PathBuf};

pub fn strip_base_paths<P>(base: P, paths: &mut Vec<PathBuf>)
where
    P: AsRef<Path>,
{
    for path in paths {
        if let Ok(stripped) = path.strip_prefix(&base) {
            *path = PathBuf::from(stripped);
        }
    }
}
