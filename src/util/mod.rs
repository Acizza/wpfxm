pub mod dir;

use std::path::{Path, PathBuf};

macro_rules! try_cont {
    ($x:expr) => {
        match $x {
            Ok(value) => value,
            _ => continue,
        }
    };
}

macro_rules! try_opt_cont {
    ($x:expr) => {
        match $x {
            Some(value) => value,
            _ => continue,
        }
    };
}

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
