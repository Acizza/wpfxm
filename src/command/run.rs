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

    let exec_name = match args.value_of("name") {
        Some(name) => name,
        None if prefix.saved_execs.len() > 1 => return Err(CommandError::NameNeededToRunExec),
        None => pfx_name,
    };

    if !prefix.saved_execs.contains_key(exec_name) {
        return Err(CommandError::ExecNotManaged(exec_name.into()));
    }

    let exec_path = &prefix.saved_execs[exec_name];

    display::info(format!("running {}", exec_name.blue()));

    let launch_opts = LaunchOptions {
        env_vars: super::parse_env_var_args(args.values_of_lossy("env_vars")),
        force_run_x86: prefix.force_run_x86 || args.is_present("force_run_x86"),
        args: args.values_of_lossy("args").unwrap_or_default(),
    };

    if let Err(err) = prefix.launch_prefix_process(config, &exec_path, launch_opts) {
        return Err(CommandError::FailedToRunProcess(
            err,
            exec_path.to_string_lossy().to_string(),
        ));
    }

    Ok(())
}
