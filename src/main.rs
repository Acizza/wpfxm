mod config;
mod error;
mod prefix;
mod util;

use crate::config::Config;
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

    let config = match Config::load(args.value_of("CONFIG")) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("err: failed to load existing config: {}", err);

            let default = Config::default();

            match default.save() {
                Ok(save_path) => {
                    println!("info: new config saved to {}", save_path.to_string_lossy())
                }
                Err(err) => eprintln!("err: failed to save new config: {}", err),
            }

            default
        }
    };

    match args.subcommand() {
        ("add", Some(args)) => manage_new_game(&config, args),
        _ => unreachable!(),
    }
}

fn manage_new_game(config: &Config, args: &clap::ArgMatches) {
    let pfx_name = args
        .value_of("PREFIX")
        .or_else(|| args.value_of("NAME"))
        .unwrap();

    if let Ok(path) = Prefix::get_data_path(pfx_name) {
        if path.exists() {
            eprintln!("{} prefix already exists! ignoring", pfx_name);
            return;
        }
    }

    let pfx_path = config.base_directory.join(pfx_name);

    let mut detected_games = {
        let mut paths = prefix::detect_unique_paths(&pfx_path);
        prefix::strip_base_paths(&pfx_path, &mut paths);
        paths
    };

    let game_path = match detected_games.len() {
        0 => {
            eprintln!("no games detected in prefix {}", pfx_name);
            return;
        }
        1 => detected_games.swap_remove(0),
        _ => {
            println!("multiple games detected!");
            unimplemented!()
        }
    };

    println!("game: {:?}", game_path);

    let prefix = Prefix::new(pfx_name, game_path);
    prefix.save().unwrap();
}
