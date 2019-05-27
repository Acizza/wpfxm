pub mod scan;

use crate::config::Config;
use crate::display::{self, ErrorSeverity};
use crate::error::PrefixError;
use crate::util::dir;
use colored::Colorize;
use hashbrown::HashMap;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

pub fn get_path<S>(config: &Config, name: S) -> PathBuf
where
    S: AsRef<str>,
{
    let name = name.as_ref();

    match config.abs_prefix_paths.get(name) {
        Some(path) => path.clone(),
        None => config.base_directory.join(name),
    }
}

pub fn is_valid<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let sys_file = path.as_ref().join("system.reg");

    let metadata = match fs::metadata(sys_file) {
        Ok(m) => m,
        Err(_) => return false,
    };

    if metadata.len() == 0 || !metadata.is_file() {
        return false;
    }

    true
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
            let line = try_cont!(line);

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

    pub fn detect<P>(path: P) -> Result<PrefixArch, PrefixError>
    where
        P: AsRef<Path>,
    {
        // These files contain the prefix architecture, and are sorted in increasing order of
        // size incase one file doesn't have the line we're looking for
        const FILES: [&str; 3] = ["userdef.reg", "user.reg", "system.reg"];

        for fname in &FILES {
            let path = path.as_ref().join(fname);

            let file = match File::open(path) {
                Ok(f) => f,
                Err(err) => {
                    display::error(ErrorSeverity::Warning, err);
                    continue;
                }
            };

            if let Some(arch) = PrefixArch::extract_from_reg_file(&file) {
                return Ok(arch);
            }
        }

        Err(PrefixError::FailedToDetectArch)
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

impl TryFrom<&str> for PrefixArch {
    type Error = ();

    fn try_from(value: &str) -> Result<PrefixArch, Self::Error> {
        let string = value.to_ascii_lowercase();

        match string.as_ref() {
            "win64" => Ok(PrefixArch::Win64),
            "win32" => Ok(PrefixArch::Win32),
            _ => Err(()),
        }
    }
}

impl Default for PrefixArch {
    fn default() -> PrefixArch {
        PrefixArch::Win64
    }
}

pub type Name = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Prefix {
    #[serde(skip)]
    pub name: String,
    pub run_hooks_explicitly: bool,
    pub arch: PrefixArch,
    pub force_run_x86: bool,
    pub saved_execs: HashMap<Name, PathBuf>,
    pub env_vars: HashMap<String, String>,
}

impl Prefix {
    fn load_direct<S, P>(name: S, path: P) -> Result<Prefix, PrefixError>
    where
        S: Into<String>,
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(&path).map_err(PrefixError::FailedToReadConfig)?;

        let mut prefix: Prefix =
            toml::from_str(&contents).map_err(PrefixError::FailedToParseConfig)?;
        prefix.name = name.into();

        Ok(prefix)
    }

    pub fn load<S>(name: S) -> Result<Prefix, PrefixError>
    where
        S: Into<String>,
    {
        let name = name.into();
        let path = Prefix::save_data_path(&name)?;

        Prefix::load_direct(name, path)
    }

    pub fn load_all() -> Result<Vec<Prefix>, PrefixError> {
        let entries =
            fs::read_dir(Prefix::save_data_dir()?).map_err(PrefixError::FailedToReadDataDir)?;

        let mut prefixes = Vec::new();

        for entry in entries {
            let entry = try_cont!(entry);
            let ftype = try_cont!(entry.file_type());

            if !ftype.is_file() {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();

            let pfx = Prefix::load_direct(name, path)?;
            prefixes.push(pfx);
        }

        Ok(prefixes)
    }

    pub fn create(&self, config: &Config) -> Result<(), PrefixError> {
        let mut cmd = Command::new("wineboot");

        self.attach_cmd_to_prefix(config, &mut cmd);
        let status = cmd.status().map_err(PrefixError::FailedToCreatePrefix)?;

        if !status.success() {
            return Err(PrefixError::WineFailedToExecute);
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), PrefixError> {
        let path = Prefix::save_data_path(&self.name)?;
        let toml = toml::to_string(self).map_err(PrefixError::FailedToSerializeConfig)?;

        fs::write(path, toml).map_err(PrefixError::FailedToWriteConfig)?;
        Ok(())
    }

    pub fn get_prefix_path(&self, config: &Config) -> PathBuf {
        get_path(config, &self.name)
    }

    pub fn save_data_dir() -> Result<PathBuf, PrefixError> {
        dir::get_data_dir().ok_or(PrefixError::FailedToGetDataDir)
    }

    pub fn save_data_path<S>(name: S) -> Result<PathBuf, PrefixError>
    where
        S: AsRef<str>,
    {
        let mut dir = Prefix::save_data_dir()?;
        dir.push(name.as_ref());
        Ok(dir)
    }

    pub fn attach_cmd_to_prefix(&self, config: &Config, cmd: &mut Command) {
        let abs_path = self.get_prefix_path(config);

        cmd.env("WINEPREFIX", &abs_path);
        cmd.env("WINEARCH", OsStr::new(self.arch.into()));
        cmd.env("WPFXM_PFX_NAME", &self.name);

        for (name, value) in &config.global_env_vars {
            cmd.env(name, value);
        }

        for (name, value) in &self.env_vars {
            cmd.env(name, value);
        }
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

    pub fn run_hooks(&self, config: &Config, hooks: &[String]) {
        for (i, hook_name) in hooks.iter().enumerate() {
            display::hook(format!(
                "running {} in {} prefix [{}/{}]",
                hook_name.green(),
                self.name.blue(),
                1 + i,
                hooks.len()
            ));

            if let Err(err) = self.run_hook_silent(hook_name, config) {
                display::error(ErrorSeverity::Warning, err);
            }
        }

        display::hook("finished running");
    }

    pub fn launch_process<P>(
        &self,
        config: &Config,
        path: P,
        opts: LaunchOptions,
    ) -> io::Result<Child>
    where
        P: AsRef<Path>,
    {
        let mut cmd = {
            let wine_process = if opts.force_run_x86 {
                "wine"
            } else {
                match self.arch {
                    PrefixArch::Win32 => "wine",
                    PrefixArch::Win64 => "wine64",
                }
            };

            Command::new(wine_process)
        };

        let path = path.as_ref();

        let exe_dir = {
            let mut path = PathBuf::from(&path);
            path.pop();
            path
        };

        cmd.arg(&path);
        cmd.args(opts.args);
        cmd.current_dir(&exe_dir);

        self.attach_cmd_to_prefix(config, &mut cmd);

        for (name, value) in opts.env_vars {
            cmd.env(name, value);
        }

        cmd.spawn()
    }

    pub fn launch_prefix_process<P>(
        &self,
        config: &Config,
        relative_path: P,
        opts: LaunchOptions,
    ) -> io::Result<Child>
    where
        P: AsRef<Path>,
    {
        let path = self.get_prefix_path(config).join(relative_path);
        self.launch_process(config, path, opts)
    }

    pub fn launch_non_wine_process<S>(
        &self,
        config: &Config,
        name: S,
        opts: LaunchOptions,
    ) -> io::Result<Child>
    where
        S: AsRef<str>,
    {
        let name = name.as_ref();
        let mut cmd = Command::new(&name);

        cmd.args(opts.args);
        self.attach_cmd_to_prefix(config, &mut cmd);

        for (name, value) in opts.env_vars {
            cmd.env(name, value);
        }

        cmd.spawn()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LaunchOptions {
    pub force_run_x86: bool,
    pub env_vars: HashMap<String, String>,
    pub args: Vec<String>,
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
        let path = Hook::get_path(&name)?;

        if !path.exists() {
            return Err(PrefixError::HookNotFound(name.as_ref().into()));
        }

        Ok(Hook { path })
    }

    pub fn get_path<S>(name: S) -> Result<PathBuf, PrefixError>
    where
        S: AsRef<str>,
    {
        let mut path = dir::get_hooks_dir()
            .ok_or(PrefixError::FailedToGetHooksDir)?
            .join(name.as_ref());

        path.set_extension("sh");
        Ok(path)
    }

    pub fn build_run_cmd(&self, config: &Config, prefix: &Prefix) -> Command {
        let mut cmd = Command::new("bash");
        cmd.arg(&self.path);
        prefix.attach_cmd_to_prefix(config, &mut cmd);

        cmd
    }
}
