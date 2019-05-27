use crate::config::Config;
use crate::display;
use crate::error::CommandError;
use crate::input;
use crate::prefix::{self, Prefix};
use colored::Colorize;
use std::path::PathBuf;

pub fn run(config: &Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    let pfx_name = args.value_of("PREFIX").unwrap();
    let exec_name = args.value_of("name").unwrap_or(pfx_name);

    manage_new_exec(config, pfx_name, exec_name)
}

pub fn manage_new_exec<S>(config: &Config, pfx_name: S, exec_name: S) -> Result<(), CommandError>
where
    S: AsRef<str>,
{
    let pfx_name = pfx_name.as_ref();

    if !Prefix::save_data_path(&pfx_name)?.exists() {
        return Err(CommandError::PrefixNotManaged(pfx_name.into()));
    }

    let mut prefix = Prefix::load(pfx_name)?;

    let exec_name = exec_name.as_ref();
    let exec_path = select_exec(config, &prefix)?;

    prefix
        .saved_execs
        .insert(exec_name.into(), exec_path.clone());

    prefix.save()?;

    display::info("setup finished");
    display::info(format!("exec name: {}", exec_name.blue()));
    display::info(format!("exec path: {}", exec_path.to_string_lossy().blue()));
    display::info(format!("prefix: {}", &prefix.name.blue()));

    Ok(())
}

fn select_exec(config: &Config, pfx: &Prefix) -> Result<PathBuf, CommandError> {
    let pfx_path = pfx.get_prefix_path(config);
    let mut found = prefix::scan::unique_executables(&pfx_path, pfx.arch);

    if found.is_empty() {
        return Err(CommandError::NoExecsDetected(pfx.name.clone()));
    }

    let formatted_paths = found
        .iter()
        .map(|e| e.to_string_lossy())
        .collect::<Vec<_>>();

    display::input(format!("found {} executable(s)", found.len()));

    let index = input::select_from_list(&formatted_paths)?;
    let exec = found.swap_remove(index);

    Ok(exec)
}
