use super::Prefix;
use std::path::{Path, PathBuf};

pub struct Applications {
    pub found: Vec<PathBuf>,
    pub common_prefix: PathBuf,
}

impl Applications {
    /// Finds all applications within the specified `prefix` and returns a new `Applications` instance with them.
    pub fn find_in_prefix(pfx: &Prefix) -> Self {
        let (mut execs, mut dirs) = super::find_files_with_extension(&pfx.path, "exe");

        while !dirs.is_empty() {
            let dir = dirs.swap_remove(0);
            let (new_execs, new_dirs) = super::find_files_with_extension(&dir, "exe");

            execs.extend(new_execs);
            dirs.extend(new_dirs);
        }

        let common_prefix = Self::find_common_path_prefix(&execs);

        Self {
            found: execs,
            common_prefix,
        }
    }

    fn find_common_path_prefix(paths: &[PathBuf]) -> PathBuf {
        let mut prefix = match paths.get(0).and_then(|p| p.parent()) {
            Some(prefix) => prefix,
            None => return PathBuf::new(),
        };

        for path in paths.iter().skip(1) {
            while let Some(parent) = prefix.parent() {
                if path.starts_with(prefix) {
                    break;
                }

                prefix = parent;
            }
        }

        prefix.into()
    }

    /// Returns an iterator over all applications, but with their prefix stripped by the common path prefix.
    pub fn stripped_iter<'a>(&'a self) -> impl Iterator<Item = &'a Path> {
        self.found
            .iter()
            .filter_map(move |app| app.strip_prefix(&self.common_prefix).ok())
    }
}
