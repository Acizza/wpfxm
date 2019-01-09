pub mod scan;

use crate::config::Config;
use crate::display::{self, ErrorSeverity};
use crate::error::PrefixError;
use crate::util::dir;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

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

impl<'a> Into<&'a str> for PrefixArch {
    fn into(self) -> &'a str {
        match self {
            PrefixArch::Win32 => "win32",
            PrefixArch::Win64 => "win64",
        }
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
                display::error(ErrorSeverity::Warning, err);
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

    pub fn attach_cmd_to_prefix(&self, config: &Config, cmd: &mut Command) {
        let abs_path = config.base_directory.join(&self.name);

        cmd.env("WINEPREFIX", &abs_path)
            .env("WINEARCH", OsStr::new(self.arch.into()))
            .env("WPFXM_PFX_NAME", &self.name);

        cmd.current_dir(abs_path.join("drive_c"));
    }

    fn run_hook_silent<S>(&self, name: S, config: &Config) -> Result<(), PrefixError>
    where
        S: AsRef<str>,
    {
        let hook = Hook::create(name)?;
        let mut cmd = hook.build_run_cmd(config, self);

        let exit_code = cmd.status().ok().and_then(|s| s.code()).unwrap_or(0);

        if exit_code != 0 {
            return Err(PrefixError::FailedToRunHook);
        }

        Ok(())
    }

    pub fn run_hook<S>(&self, name: S, config: &Config) -> Result<(), PrefixError>
    where
        S: AsRef<str>,
    {
        display::hook(format!("running {}", name.as_ref()));
        self.run_hook_silent(name, config)
    }

    pub fn run_hooks(&self, config: &Config, hooks: &[String]) {
        for (i, hook_name) in hooks.iter().enumerate() {
            display::hook(format!("running {} [{}/{}]", hook_name, 1 + i, hooks.len()));

            match self.run_hook_silent(hook_name, config) {
                Ok(_) => (),
                Err(err) => display::error(ErrorSeverity::Warning, err),
            }
        }
    }
}

#[derive(Debug)]
pub struct Hook {
    pub path: PathBuf,
}

impl Hook {
    pub fn create<S>(name: S) -> Result<Hook, PrefixError>
    where
        S: AsRef<str>,
    {
        let mut path = dir::get_hooks_dir()
            .ok_or(PrefixError::FailedToGetHooksDir)?
            .join(name.as_ref());

        path.set_extension("sh");

        Ok(Hook { path })
    }

    pub fn build_run_cmd(&self, config: &Config, prefix: &Prefix) -> Command {
        let mut cmd = Command::new("bash");
        cmd.arg(&self.path);
        prefix.attach_cmd_to_prefix(config, &mut cmd);

        cmd
    }
}