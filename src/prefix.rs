use crate::error::PrefixError;
use crate::util::dir;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_ROOT_FOLDERS: [&str; 6] = [
    "Program Files",
    "Program Files (x86)",
    "ProgramData",
    "users",
    "windows",
    ".windows-serial",
];

const DEFAULT_PROGRAM_FILES: [&str; 4] = [
    "Common Files",
    "Internet Explorer",
    "Windows Media Player",
    "Windows NT",
];

fn detect_dir_diff(path: &Path, exclude_paths: &[&str], results: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("failed to read {:?}: {}", path, err);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("failed to read {:?}: {}", path, err);
                continue;
            }
        };

        let mut skip_file = false;

        for &excluded_path in exclude_paths {
            if entry.file_name().to_string_lossy() == excluded_path {
                skip_file = true;
                break;
            }
        }

        if skip_file {
            continue;
        }

        results.push(entry.path());
    }
}

pub fn detect_unique_paths<P>(pfx: P) -> Vec<PathBuf>
where
    P: Into<PathBuf>,
{
    let pfx = pfx.into().join("drive_c");
    let mut paths = Vec::with_capacity(1);

    detect_dir_diff(&pfx, &DEFAULT_ROOT_FOLDERS, &mut paths);
    detect_dir_diff(
        &pfx.clone().join("Program Files"),
        &DEFAULT_PROGRAM_FILES,
        &mut paths,
    );
    detect_dir_diff(
        &pfx.clone().join("Program Files (x86)"),
        &DEFAULT_PROGRAM_FILES,
        &mut paths,
    );

    paths
}

pub fn strip_base_paths<P>(base: P, paths: &mut Vec<PathBuf>)
where
    P: Into<PathBuf>,
{
    let base = base.into();

    for path in paths {
        if let Ok(stripped) = path.strip_prefix(&base) {
            *path = PathBuf::from(stripped);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prefix {
    #[serde(skip)]
    pub name: String,
    pub game_path: PathBuf,
}

impl Prefix {
    pub fn new<S, P>(name: S, game_path: P) -> Prefix
    where
        S: Into<String>,
        P: Into<PathBuf>,
    {
        Prefix {
            name: name.into(),
            game_path: game_path.into(),
        }
    }

    pub fn load<'a, S>(name: S) -> Result<Prefix, PrefixError>
    where
        S: Into<Cow<'a, str>>,
    {
        let name = name.into();

        let path = Prefix::get_data_path(&name)?;
        let contents = fs::read_to_string(&path).map_err(PrefixError::FailedToReadConfig)?;

        let mut prefix: Prefix =
            toml::from_str(&contents).map_err(PrefixError::FailedToParseConfig)?;
        prefix.name = name.to_string();

        Ok(prefix)
    }

    pub fn save(&self) -> Result<(), PrefixError> {
        let path = Prefix::get_data_path(&self.name)?;
        let toml = toml::to_string_pretty(self).map_err(PrefixError::FailedToSerializeConfig)?;

        fs::write(path, toml).map_err(PrefixError::FailedToWriteConfig)?;
        Ok(())
    }

    pub fn get_data_path<S>(name: S) -> Result<PathBuf, PrefixError>
    where
        S: AsRef<str>,
    {
        let mut dir = dir::get_valid_data_dir().ok_or(PrefixError::FailedToGetDataDir)?;
        dir.push(name.as_ref());
        Ok(dir)
    }
}
