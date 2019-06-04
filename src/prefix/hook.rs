use super::Prefix;
use crate::config::Config;
use crate::display::{self, ErrorSeverity};
use crate::error::PrefixError;
use crate::util::dir;
use colored::Colorize;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct Hook {
    pub path: PathBuf,
}

impl Hook {
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

    pub fn run(&self, config: &Config, prefix: &Prefix) -> Result<(), PrefixError> {
        let mut cmd = self.build_run_cmd(config, prefix);
        let exit_code = cmd.status().ok().and_then(|s| s.code()).unwrap_or(0);

        if exit_code != 0 {
            return Err(PrefixError::FailedToRunHook);
        }

        Ok(())
    }
}

impl TryFrom<&str> for Hook {
    type Error = PrefixError;

    fn try_from(name: &str) -> Result<Hook, Self::Error> {
        let path = Hook::get_path(name)?;

        if !path.exists() {
            return Err(PrefixError::HookNotFound(name.into()));
        }

        Ok(Hook { path })
    }
}

pub fn run_list(config: &Config, prefix: &Prefix, names: &[String]) {
    for (i, name) in names.iter().enumerate() {
        let hook = match Hook::try_from(name.as_ref()) {
            Ok(hook) => hook,
            Err(err) => {
                display::error(ErrorSeverity::Warning, err);
                continue;
            }
        };

        display::hook(format!(
            "running {} in {} prefix [{}/{}]",
            name.green(),
            prefix.name.blue(),
            1 + i,
            names.len()
        ));

        if let Err(err) = hook.run(config, prefix) {
            display::error(ErrorSeverity::Warning, err);
        }
    }

    display::hook("finished running");
}
