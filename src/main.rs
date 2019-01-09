mod config;
mod error;
mod log;
mod prefix;
mod util;

use crate::config::Config;
use crate::error::Error;
use crate::log::ErrorSeverity;
use crate::prefix::Prefix;

fn main() {
    use clap::{clap_app, AppSettings};

    let args = clap_app!(wpfxm =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "A simple tool to manage Wine prefixes for games")
        (@subcommand add =>
            (about: "Manage a new game through wpfxm")
            (@arg NAME: +takes_value +required "The name to reference the game by")
            (@arg PREFIX: +takes_value "The name of the Wine prefix the game is in, relative to the base folder")
        )
    )
    .setting(AppSettings::SubcommandRequired)
    .get_matches();

    match run(&args) {
        Ok(_) => (),
        Err(err) => {
            log::error(ErrorSeverity::Fatal, err);
            std::process::exit(1);
        }
    }
}

fn run(args: &clap::ArgMatches) -> Result<(), Error> {
    let config = match Config::load(args.value_of("CONFIG")) {
        Ok(config) => config,
        Err(err) => {
            log::error(ErrorSeverity::Warning, err);

            let default = Config::default();

            match default.save() {
                Ok(save_path) => {
                    log::info(format!(
                        "new config saved to {}",
                        save_path.to_string_lossy()
                    ));
                }
                Err(err) => log::error(ErrorSeverity::Warning, err),
            }

            default
        }
    };

    match args.subcommand() {
        ("add", Some(args)) => manage_new_game(&config, args),
        _ => unreachable!(),
    }
}

fn manage_new_game(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
    let pfx_name = args
        .value_of("PREFIX")
        .or_else(|| args.value_of("NAME"))
        .unwrap();

    if let Ok(path) = Prefix::get_data_path(pfx_name) {
        if path.exists() {
            return Err(Error::PrefixAlreadyManaged(pfx_name.into()));
        }
    }

    let pfx_path = config.base_directory.join(pfx_name);
    let arch = prefix::detect_arch(&pfx_path)?;

    let mut detected_games = {
        let mut paths = prefix::detect_unique_paths(&pfx_path, arch);
        prefix::strip_base_paths(&pfx_path, &mut paths);
        paths
    };

    let game_path = match detected_games.len() {
        0 => return Err(Error::NoGamesDetected(pfx_name.into())),
        1 => detected_games.swap_remove(0),
        _ => {
            println!("multiple games detected!");
            unimplemented!()
        }
    };

    log::info(format!(
        "detected game in \"{}\" prefix at \"{}\"",
        pfx_name,
        game_path.to_string_lossy()
    ));

    let prefix = Prefix::new(pfx_name, game_path, arch);
    prefix.save()?;
    prefix.run_hooks(config, &config.setup_hooks);

    Ok(())
}
