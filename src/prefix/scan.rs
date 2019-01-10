use super::PrefixArch;
use crate::display::{self, ErrorSeverity};
use crate::error::PrefixError;
use crate::util;
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
            display::error(
                ErrorSeverity::Warning,
                PrefixError::FailedToReadPath(err, path.to_string_lossy().into()),
            );

            return;
        }
    };

    for entry in entries {
        let entry = try_cont!(entry);
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
    P: AsRef<Path>,
{
    let pfx = pfx.as_ref().join("drive_c");
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
        let entry = try_cont!(entry);
        let ftype = try_cont!(entry.file_type());
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

pub fn unique_executables<P>(pfx: P, arch: PrefixArch) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    const MAX_SEARCH_DEPTH: u8 = 4;

    let paths = unique_paths(&pfx, arch);
    let mut executables = Vec::new();

    for path in paths {
        let found = find_executables(path, MAX_SEARCH_DEPTH);
        executables.extend(found);
    }

    util::strip_base_paths(&pfx, &mut executables);
    executables
}
