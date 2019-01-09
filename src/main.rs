mod config;
mod display;
mod error;
mod input;
mod prefix;
mod util;

use crate::config::Config;
use crate::display::ErrorSeverity;
use crate::error::{Error, PrefixError};
use crate::prefix::{Prefix, PrefixArch};
use colored::Colorize;
use std::path::{Path, PathBuf};

fn main() {
    use clap::{clap_app, AppSettings};

    let args = clap_app!(wpfxm =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "A simple tool to manage Wine prefixes for games")
        (@subcommand add =>
            (about: "Manage a new game through wpfxm")
            (@arg PREFIX: +takes_value +required "The Wine prefix to look for games in, relative to the base folder")
        )
        (@subcommand run =>
            (about: "Run a game in a prefix managed by wpfxm")
            (@arg PREFIX: +takes_value +required "The name of the Wine prefix")
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

fn manage_new_game(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
    let pfx_name = args.value_of("PREFIX").unwrap();

    if let Ok(path) = Prefix::get_data_path(pfx_name) {
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

    let prefix = Prefix::new(pfx_name, game_path, arch);
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

    if let Err(err) = prefix.launch_process(config, &prefix.game_path) {
        return Err(Error::FailedToRunGame(
            err,
            prefix.game_path.to_string_lossy().to_string(),
        ));
    }

    Ok(())
}
