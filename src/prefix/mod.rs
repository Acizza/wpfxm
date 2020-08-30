use crate::util;
use anyhow::{anyhow, Context, Result};
use std::fmt;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Prefix {
    pub name: String,
    pub arch: Arch,
    pub path: PathBuf,
    pub found_applications: Vec<PathBuf>,
}

impl Prefix {
    pub fn from_path<P, S>(path: P, name: S) -> Result<Self>
    where
        P: Into<PathBuf>,
        S: Into<String>,
    {
        let path = path.into();

        if !is_valid(&path) {
            return Err(anyhow!("{} is not a valid Wine prefix", path.display()));
        }

        let arch = Arch::from_prefix(&path).with_context(|| {
            anyhow!(
                "failed to detect prefix architecture for {}",
                path.display()
            )
        })?;

        Ok(Self {
            name: name.into(),
            arch,
            path,
            found_applications: Vec::new(),
        })
    }

    pub fn all_from_dir<P>(dir: P) -> Result<Vec<Self>>
    where
        P: AsRef<Path>,
    {
        let prefixes = util::subdirectories(dir)?
            .into_iter()
            .filter_map(|dir| {
                let name = dir.file_name();
                let name = name.to_string_lossy();
                Self::from_path(dir.path(), name).ok()
            })
            .collect();

        Ok(prefixes)
    }

    fn find_execs_and_dirs(path: &Path, base: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
        const EXCLUDE_FOLDERS: [&str; 4] = [
            "windows",
            "windows nt",
            "windows media player",
            "internet explorer",
        ];

        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => return (Vec::new(), Vec::new()),
        };

        let mut execs = Vec::new();
        let mut dirs = Vec::new();

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                _ => continue,
            };

            let file_type = match entry.file_type() {
                Ok(ftype) => ftype,
                _ => continue,
            };

            let filename = entry.file_name();
            let filename = filename.to_string_lossy();

            if file_type.is_dir() {
                if EXCLUDE_FOLDERS
                    .iter()
                    .any(|&exclude| filename.eq_ignore_ascii_case(exclude))
                {
                    continue;
                }

                dirs.push(entry.path());
                continue;
            }

            if !filename.ends_with(".exe") {
                continue;
            }

            let path = util::strip_base_path(base, entry.path());
            execs.push(path);
        }

        (execs, dirs)
    }

    /// Finds all executables within the prefix and returns their relative path.
    ///
    /// Implementation note: a linear solution is used to scan directories instead of a recursive one,
    /// so there isn't a stack overflow risk.
    pub fn find_relative_executables(&self) -> Vec<PathBuf> {
        let (mut execs, mut dirs) = Self::find_execs_and_dirs(&self.path, &self.path);

        while !dirs.is_empty() {
            let dir = dirs.swap_remove(0);
            let (new_execs, new_dirs) = Self::find_execs_and_dirs(&dir, &self.path);

            execs.extend(new_execs);
            dirs.extend(new_dirs);
        }

        execs
    }

    pub fn populate_applications(&mut self) {
        self.found_applications = self.find_relative_executables().into_iter().collect();
    }
}

impl AsRef<str> for Prefix {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub enum Arch {
    X86,
    X86_64,
}

impl Arch {
    fn from_file(file: &File) -> Option<Self> {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                _ => return None,
            };

            match line.splitn(2, "#arch=").nth(1) {
                Some("win32") => return Some(Self::X86),
                Some("win64") => return Some(Self::X86_64),
                _ => (),
            }
        }

        None
    }

    pub fn from_prefix<P>(path: P) -> Option<Self>
    where
        P: AsRef<Path>,
    {
        // These files contain the prefix architecture, and are sorted in increasing order of
        // size incase one file doesn't have the line we're looking for
        const FILES: [&str; 3] = ["userdef.reg", "user.reg", "system.reg"];

        for file in &FILES {
            let path = path.as_ref().join(file);

            let file = match File::open(path) {
                Ok(f) => f,
                _ => continue,
            };

            if let Some(arch) = Arch::from_file(&file) {
                return Some(arch);
            }
        }

        None
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X86 => write!(f, "x86"),
            Self::X86_64 => write!(f, "x86-64"),
        }
    }
}

pub fn is_valid<P>(path: P) -> bool
where
    P: AsRef<Path> + Into<PathBuf>,
{
    let sys_reg_file = path.as_ref().join("system.reg");

    if !sys_reg_file.exists() {
        return false;
    }

    let drive_c_path = {
        let mut path = path.into();
        path.push("drive_c");
        path
    };

    if !drive_c_path.exists() {
        return false;
    }

    let drive_metadata = match fs::metadata(drive_c_path) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };

    drive_metadata.is_dir()
}
