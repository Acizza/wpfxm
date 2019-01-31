use crate::config::Config;
use crate::display;
use crate::error::{CommandError, PrefixError};
use crate::prefix::{self, LaunchOptions, Prefix};
use colored::Colorize;

pub fn run(config: &Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    let pfx_name = args.value_of("PREFIX").unwrap();

    if !prefix::get_path(config, &pfx_name).exists() {
        return Err(CommandError::PrefixDoesNotExist);
    }

    let pfx = match Prefix::load(pfx_name) {
        Ok(pfx) => pfx,
        Err(PrefixError::FailedToReadConfig(_)) => {
            return Err(CommandError::PrefixNotManaged(pfx_name.into()));
        }
        Err(err) => return Err(err.into()),
    };

    let mut run_args = args.values_of_lossy("ARGS").unwrap();
    let exe_name = run_args.remove(0);

    display::info(format!("running [{}]", exe_name.blue()));

    let opts = LaunchOptions {
        force_run_x86: false, // This doesn't matter when running a non-Wine process
        env_vars: super::parse_env_var_args(args.values_of_lossy("env_vars")),
        args: run_args,
    };

    pfx.launch_non_wine_process(config, &exe_name, opts)
        .map_err(|err| CommandError::FailedToRunProcess(err, exe_name))?;

    Ok(())
}
