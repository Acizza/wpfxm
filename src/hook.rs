use crate::config::Config;
use crate::error::HookError;
use crate::prefix::Prefix;
use crate::util::dir;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct Hook {
    pub path: PathBuf,
}

impl Hook {
    pub fn create<S>(name: S) -> Result<Hook, HookError>
    where
        S: AsRef<str>,
    {
        let mut path = dir::get_hooks_dir()
            .ok_or(HookError::FailedToGetHooksDir)?
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
