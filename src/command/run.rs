use crate::config::Config;
use crate::display;
use crate::error::{CommandError, PrefixError};
use crate::prefix::{LaunchOptions, Prefix};
use colored::Colorize;

pub fn run(config: &Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    let pfx_name = args.value_of("PREFIX").unwrap();

    let prefix = match Prefix::load(pfx_name) {
        Ok(pfx) => pfx,
        Err(PrefixError::FailedToReadConfig(_)) => {
            return Err(CommandError::PrefixNotManaged(pfx_name.into()));
        }
        Err(err) => return Err(err.into()),
    };

    // TODO: simplify
    let (exec_name, exec_path) = match prefix.saved_execs.len() {
        0 => return Err(CommandError::NoSavedExecs),
        1 => {
            let (name, value) = prefix.saved_execs.iter().next().unwrap();
            (name.clone(), value.clone())
        }
        _ => {
            let exec_name = args
                .value_of("name")
                .ok_or(CommandError::NameNeededToRunExec)?;

            if !prefix.saved_execs.contains_key(exec_name) {
                return Err(CommandError::ExecNotManaged(exec_name.into()))?;
            }

            let value = prefix.saved_execs[exec_name].clone();
            (exec_name.into(), value)
        }
    };

    display::info(format!("running {}", exec_name.blue()));

    let launch_opts = LaunchOptions {
        env_vars: super::parse_env_var_args(args.values_of_lossy("env_vars")),
        force_run_x86: prefix.force_run_x86 || args.is_present("force_run_x86"),
        args: Vec::new(),
    };

    if let Err(err) = prefix.launch_prefix_process(config, &exec_path, launch_opts) {
        return Err(CommandError::FailedToRunProcess(
            err,
            exec_path.to_string_lossy().to_string(),
        ));
    }

    Ok(())
}
