mod config;
mod error;
mod prefix;

use crate::config::Config;

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
    let pfx = {
        let pfx_name = args
            .value_of("PREFIX")
            .or_else(|| args.value_of("NAME"))
            .unwrap();

        config.base_directory.join(pfx_name)
    };

    let detected_games = {
        let mut paths = prefix::detect_unique_paths(&pfx);
        prefix::strip_base_paths(&pfx, &mut paths);
        paths
    };

    println!("{:?}", detected_games);
}
