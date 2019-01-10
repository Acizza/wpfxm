#[macro_use]
mod util;

mod config;
mod display;
mod error;
mod input;
mod prefix;

use crate::config::Config;
use crate::display::ErrorSeverity;
use crate::error::{Error, PrefixError};
use crate::prefix::{LaunchOptions, Prefix, PrefixArch};
use colored::Colorize;
use std::path::{Path, PathBuf};

fn main() {
    use clap::{clap_app, AppSettings};

    let args = clap_app!(wpfxm =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "A simple tool to manage Wine prefixes for games")
        (@subcommand add =>
            (about: "Manage a new prefix through wpfxm")
            (@arg PREFIX: +takes_value +required "The Wine prefix to look for applications in, relative to the base folder")
            (@arg env_vars: -e --env +takes_value +multiple "The environment variables to always use with this prefix")
            (@arg force_run_x86: --x86 "Run all applications in this prefix in 32-bit mode")
        )
        (@subcommand run =>
            (about: "Run an application in a prefix managed by wpfxm")
            (@arg PREFIX: +takes_value +required "The name of the Wine prefix")
            (@arg env_vars: -e --env +takes_value +multiple "The environment variables to launch with")
            (@arg force_run_x86: --x86 "Run the application in 32-bit mode")
        )
        (@subcommand hook =>
            (about: "Manage hooks for a prefix")
            (@subcommand run =>
                (about: "Runs a hook in all managed Wine prefixes (unless -p is specified)")
                (@arg HOOK: +takes_value +required "The name of the hook")
                (@arg prefix: -p --prefix +takes_value "The prefix to run the hook in")
            )
        )
    )
    .setting(AppSettings::SubcommandRequired)
    .get_matches();

    match run(&args) {
        Ok(_) => (),
        Err(err) => {
            display::error(ErrorSeverity::Fatal, err);
            std::process::exit(1);
        }
    }
}

fn run(args: &clap::ArgMatches) -> Result<(), Error> {
    let config = match Config::load(args.value_of("CONFIG")) {
        Ok(config) => config,
        Err(err) => {
            display::error(ErrorSeverity::Warning, err);

            let default = Config::default();

            match default.save() {
                Ok(save_path) => {
                    display::info(format!(
                        "new config saved to {}",
                        save_path.to_string_lossy()
                    ));
                }
                Err(err) => display::error(ErrorSeverity::Warning, err),
            }

            default
        }
    };

    match args.subcommand() {
        ("add", Some(args)) => manage_new_game(&config, args),
        ("run", Some(args)) => run_game(&config, args),
        ("hook", Some(args)) => hook::dispatch_command(&config, args),
        _ => unreachable!(),
    }
}

fn select_game_in_prefix<P, S>(pfx: P, pfx_name: S, arch: PrefixArch) -> Result<PathBuf, Error>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut found = prefix::scan::unique_executables(pfx, arch);

    if found.is_empty() {
        return Err(Error::NoGamesDetected(pfx_name.as_ref().to_string()));
    }

    let formatted_paths = found
        .iter()
        .map(|e| e.to_string_lossy())
        .collect::<Vec<_>>();

    display::input(format!("found {} game(s)", found.len()));

    let index = input::select_from_list(&formatted_paths)?;
    let game = found.swap_remove(index);

    Ok(game)
}

fn parse_env_var_arg<S>(arg: S) -> Option<(String, String)>
where
    S: AsRef<str>,
{
    let split = arg.as_ref().splitn(2, '=').collect::<Vec<_>>();

    if split.len() < 2 {
        return None;
    }

    let name = split[0].to_string();
    let value = split[1].to_string();

    Some((name, value))
}

fn parse_env_var_args(args: &clap::ArgMatches) -> Vec<(String, String)> {
    args.values_of_lossy("env_vars")
        .map(|ev| ev.iter().filter_map(parse_env_var_arg).collect())
        .unwrap_or_else(Vec::new)
}

fn manage_new_game(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
    let pfx_name = args.value_of("PREFIX").unwrap();

    if let Ok(path) = Prefix::get_data_file(pfx_name) {
        if path.exists() {
            return Err(Error::PrefixAlreadyManaged(pfx_name.into()));
        }
    }

    let pfx_path = config.base_directory.join(pfx_name);
    let arch = prefix::detect_arch(&pfx_path)?;
    let game_path = select_game_in_prefix(pfx_path, pfx_name, arch)?;

    display::info(format!(
        "prefix \"{}\" will launch \"{}\"",
        pfx_name,
        game_path.to_string_lossy()
    ));

    let prefix = Prefix {
        name: pfx_name.into(),
        game_path,
        arch,
        env_vars: parse_env_var_args(args),
        force_run_x86: args.is_present("force_run_x86"),
    };

    prefix.save()?;
    prefix.run_hooks(config, &config.setup_hooks);

    Ok(())
}

fn run_game(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
    let pfx_name = args.value_of("PREFIX").unwrap();

    let prefix = match Prefix::load(pfx_name) {
        Ok(pfx) => pfx,
        Err(PrefixError::FailedToReadConfig(_)) => {
            return Err(Error::PrefixNotManaged(pfx_name.into()))
        }
        Err(err) => return Err(err.into()),
    };

    display::info(format!(
        "running [{}]",
        prefix.game_path.to_string_lossy().blue()
    ));

    let launch_opts = LaunchOptions {
        env_vars: parse_env_var_args(args),
        force_run_x86: prefix.force_run_x86 || args.is_present("force_run_x86"),
    };

    if let Err(err) = prefix.launch_process(config, &prefix.game_path, launch_opts) {
        return Err(Error::FailedToRunGame(
            err,
            prefix.game_path.to_string_lossy().to_string(),
        ));
    }

    Ok(())
}

mod hook {
    use super::*;

    pub fn dispatch_command(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
        match args.subcommand() {
            ("run", Some(args)) => run_hook(config, args),
            _ => unreachable!(),
        }
    }

    fn run_hook(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
        let hook_name = args.value_of("HOOK").unwrap();

        match args.value_of("prefix") {
            Some(pfx_name) => {
                let prefix = Prefix::load(pfx_name)?;
                prefix.run_hook(hook_name, config)?;
            }
            None => {
                let prefixes = Prefix::load_all()?;

                for prefix in prefixes {
                    prefix.run_hook(hook_name, config)?;
                }
            }
        }

        Ok(())
    }
}
