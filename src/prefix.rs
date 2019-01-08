use crate::error::PrefixError;
use crate::log::{self, ErrorSeverity};
use crate::util::dir;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
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

pub fn detect_unique_paths<P>(pfx: P, arch: PrefixArch) -> Vec<PathBuf>
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

    if arch != PrefixArch::Win32 {
        detect_dir_diff(
            &pfx.clone().join("Program Files (x86)"),
            &DEFAULT_PROGRAM_FILES,
            &mut paths,
        );
    }

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

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum PrefixArch {
    Win32,
    Win64,
}

impl PrefixArch {
    fn extract_from_reg_file(file: &File) -> Option<PrefixArch> {
        const ARCH_STR: &str = "#arch=";
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };

            if !line.contains(ARCH_STR) {
                continue;
            }

            match line.splitn(2, ARCH_STR).nth(1) {
                Some("win32") => return Some(PrefixArch::Win32),
                Some("win64") => return Some(PrefixArch::Win64),
                _ => (),
            }
        }

        None
    }
}

pub fn detect_arch<P>(path: P) -> Result<PrefixArch, PrefixError>
where
    P: AsRef<Path>,
{
    // These files contain the prefix architecture, and are sorted in increasing order of
    // size incase one file doesn't have the line we're looking for
    const FILES: [&str; 3] = ["userdef.reg", "user.reg", "system.reg"];
    let mut arch = None;

    for fname in &FILES {
        let path = path.as_ref().join(fname);

        let file = match File::open(path) {
            Ok(f) => f,
            Err(err) => {
                log::error(ErrorSeverity::Warning, err);
                continue;
            }
        };

        if let Some(detected) = PrefixArch::extract_from_reg_file(&file) {
            arch = Some(detected);
            break;
        }
    }

    let arch = match arch {
        Some(arch) => arch,
        None => return Err(PrefixError::FailedToDetectArch),
    };

    Ok(arch)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prefix {
    #[serde(skip)]
    pub name: String,
    pub game_path: PathBuf,
    pub arch: PrefixArch,
}

impl Prefix {
    pub fn new<S, P>(name: S, game_path: P, arch: PrefixArch) -> Prefix
    where
        S: Into<String>,
        P: Into<PathBuf>,
    {
        Prefix {
            name: name.into(),
            game_path: game_path.into(),
            arch,
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
        let mut dir = dir::get_data_dir().ok_or(PrefixError::FailedToGetDataDir)?;
        dir.push(name.as_ref());
        Ok(dir)
    }
}
