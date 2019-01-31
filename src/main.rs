#[macro_use]
mod util;

mod command;
mod config;
mod display;
mod error;
mod input;
mod prefix;

use crate::config::Config;
use crate::display::ErrorSeverity;
use crate::error::Error;

fn main() {
    use clap::{clap_app, AppSettings};

    let args = clap_app!(wpfxm =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "A simple tool to manage Wine prefixes for games")
        (@subcommand new =>
            (about: "Create a new Wine prefix through wpfxm")
            (@arg PREFIX: +takes_value +required "The name of the Wine prefix")
            (@arg arch: --arch +takes_value "The architecture to use for the prefix")
            (@arg env_vars: -e --env +takes_value +multiple "The environment variables to use for the prefix")
            (@arg run: -r --run +takes_value +multiple "The program to run after prefix creation")
            (@arg add: -a --add +takes_value "Shortcut for running the add command after running the command specified by -r. This is ignored if -r is not specified")
            (@arg force_run_x86: --x86 "Run all applications in this prefix as 32-bit, even if the prefix is 64-bit")
            (@arg path: -p --path +takes_value "The absolute path of the prefix. This is useful for prefixes that you want to manage outside of the base directory.")
            (@arg explicit_hooks: --explicithooks "Only run hooks when called directly via the hook run command. This will avoid the setup hooks running on this prefix, and will be exempt from generic hook run calls.")
        )
        (@subcommand add =>
            (about: "Scan an existing prefix for an application to manage")
            (@arg PREFIX: +takes_value +required "The Wine prefix to look for applications in, relative to the base folder")
            (@arg NAME: +takes_value +required "The name to refer to the added application")
        )
        (@subcommand run =>
            (about: "Run a saved application in a prefix managed by wpfxm")
            (@arg PREFIX: +takes_value +required "The name of the Wine prefix")
            (@arg name: +takes_value "The name of the application to run; can be omitted if there is only one application managed")
            (@arg env_vars: -e --env +takes_value +multiple "The environment variables to launch with")
            (@arg force_run_x86: --x86 "Run the application in 32-bit mode")
        )
        (@subcommand exec =>
            (about: "Run an arbitrary executable for a prefix managed by wpfxm")
            (@arg PREFIX: +takes_value +required "The prefix to run the executable in")
            (@arg ARGS: +takes_value +multiple +required "The executable to launch")
            (@arg env_vars: -e --env +takes_value +multiple "The environment variables to launch with the executable")
        )
        (@subcommand hook =>
            (about: "Manage hooks for a prefix")
            (@subcommand run =>
                (about: "Runs hooks globally or in a specified prefix")
                (@arg HOOKS: +takes_value +multiple +required "The list of hooks to run")
                (@arg prefix: -p --prefix +takes_value "The prefix to run the hooks in")
            )
        )
        (@subcommand ls =>
            (about: "Lists prefixes and applications managed by wpfxm")
            (@arg prefix: +takes_value "The prefix to show")
        )
        (@subcommand rm =>
            (about: "Remove a prefix managed by wpfxm")
            (@arg PREFIX: +takes_value +required "The name of the prefix to remove")
            (@arg data_only: -d --data "Remove the prefix save data, but keep the prefix itself")
            (@arg pfx_only: -p --prefix "Remove the prefix, but keep the save data")
        )
        (@subcommand cfg =>
            (about: "Manage configuration globally or for a specific prefix")
            (@subcommand set =>
                (about: "Set a config setting, overwriting the previous value")
                (@subcommand baseDir =>
                    (about: "Set the directory that will contain managed Wine prefixes")
                    (@arg PATH: +takes_value +required "The path to the base directory")
                )
                (@subcommand setupHooks =>
                    (about: "Set the list of hooks to run when a new prefix is added")
                    (@arg HOOKS: +takes_value +multiple +required "The list of hooks")
                    (@arg append: -a --append "Append the list of hooks, instead of setting them")
                )
                (@subcommand env =>
                    (about: "Set the list of environment variables to use")
                    (@arg VARS: +takes_value +multiple +required "The list of variables, formatted as NAME=VALUE")
                    (@arg prefix: -p --prefix +takes_value "The prefix to apply the variables to")
                    (@arg append: -a --append "Append the list of environment variables, instead of setting them")
                )
                (@subcommand forceRunX86 =>
                    (about: "Force a prefix to always run an application in 32-bit mode")
                    (@arg PREFIX: +takes_value +required "The prefix to enable the setting in")
                    (@arg ENABLE: +takes_value +required "Possible values are true and false")
                )
                (@subcommand explicitHooks =>
                    (about: "Set whether or not hooks will be executed for the specified prefix during generic hook run calls")
                    (@arg PREFIX: +takes_value +required "The prefix to apply the setting to")
                    (@arg ENABLE: +takes_value +required "Possible values are true and false")
                )
            )
            (@subcommand rm =>
                (about: "Remove a global or prefix setting")
                (@subcommand setupHooks =>
                    (about: "Remove setup hooks")
                    (@arg hooks: +takes_value +multiple "The hooks to remove")
                )
                (@subcommand env =>
                    (about: "Remove environment variables globally or from a certain prefix")
                    (@arg vars: +takes_value +multiple "The variables to clear")
                    (@arg prefix: -p --prefix +takes_value "The prefix to clear the variables from")
                )
            )
        )
    )
    .setting(AppSettings::SubcommandRequiredElseHelp)
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
    let mut config = match Config::load(args.value_of("CONFIG")) {
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
        ("new", Some(args)) => command::new::run(&mut config, args)?,
        ("add", Some(args)) => command::add::run(&config, args)?,
        ("run", Some(args)) => command::run::run(&config, args)?,
        ("exec", Some(args)) => command::exec::run(&config, args)?,
        ("hook", Some(args)) => command::hook::run(&config, args)?,
        ("ls", Some(args)) => command::ls::run(args)?,
        ("rm", Some(args)) => command::rm::run(&mut config, args)?,
        ("cfg", Some(args)) => command::cfg::run(&mut config, args)?,
        _ => unreachable!(),
    }

    Ok(())
}
