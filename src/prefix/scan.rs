use super::PrefixArch;
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

fn dir_diff(path: &Path, exclude_paths: &[&str], results: &mut Vec<PathBuf>) {
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

pub fn unique_paths<P>(pfx: P, arch: PrefixArch) -> Vec<PathBuf>
where
    P: Into<PathBuf>,
{
    let pfx = pfx.into().join("drive_c");
    let mut paths = Vec::with_capacity(1);

    dir_diff(&pfx, &DEFAULT_ROOT_FOLDERS, &mut paths);
    dir_diff(
        &pfx.clone().join("Program Files"),
        &DEFAULT_PROGRAM_FILES,
        &mut paths,
    );

    if arch != PrefixArch::Win32 {
        dir_diff(
            &pfx.clone().join("Program Files (x86)"),
            &DEFAULT_PROGRAM_FILES,
            &mut paths,
        );
    }

    paths
}

pub fn find_executables<P>(path: P, max_depth: u8) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    if max_depth == 0 {
        return Vec::new();
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut executables = Vec::new();
    let mut directories = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let ftype = match entry.file_type() {
            Ok(ftype) => ftype,
            Err(_) => continue,
        };

        let path = entry.path();

        if ftype.is_dir() {
            directories.push(path);
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext.to_string_lossy() == "exe" {
                executables.push(path);
            }
        }
    }

    for dir in directories {
        executables.extend(find_executables(dir, max_depth - 1));
    }

    executables
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
