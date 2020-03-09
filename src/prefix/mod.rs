pub mod application;

use crate::err::{self, Error, Result};
use crate::util;
use application::Application;
use snafu::OptionExt;
use std::fmt;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Prefix {
    pub name: String,
    pub arch: Arch,
    pub applications: Vec<Application>,
}

impl Prefix {
    pub fn from_path<P, S>(path: P, name: S) -> Result<Self>
    where
        P: AsRef<Path>,
        S: Into<String>,
    {
        let path = path.as_ref();

        if !is_valid(&path) {
            return Err(Error::NotAPrefix { path: path.into() });
        }

        let arch = Arch::from_prefix(&path).context(err::NoArchDetected { path })?;

        Ok(Self {
            name: name.into(),
            arch,
            applications: Vec::new(),
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
