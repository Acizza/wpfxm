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
use std::collections::HashMap;
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
        (@subcommand config =>
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
                )
                (@subcommand env =>
                    (about: "Set the list of environment variables to use")
                    (@arg VARS: +takes_value +multiple +required "The list of variables, formatted as NAME=VALUE")
                    (@arg prefix: -p --prefix +takes_value "The prefix to apply the variables to")
                )
                (@subcommand forceRunX86 =>
                    (about: "Force a prefix to always run an application in 32-bit mode")
                    (@arg PREFIX: +takes_value +required "The prefix to enable the setting in")
                    (@arg ENABLE: +takes_value +required "Possible values are true and false")
                )
            )
            (@subcommand add =>
                (about: "Append a config setting, adding to the previous value")
                (@subcommand setupHooks =>
                    (about: "Append the list of hooks to run when a new prefix is added")
                    (@arg HOOKS: +takes_value +multiple +required "The list of hooks")
                )
                (@subcommand env =>
                    (about: "Append the list of environment variables to use")
                    (@arg VARS: +takes_value +multiple +required "The list of variables, formatted as NAME=VALUE")
                    (@arg prefix: -p --prefix +takes_value "The prefix to append the variables to")
                )
            )
            (@subcommand remove =>
                (about: "Remove a value from a config setting")
                (@subcommand setupHooks =>
                    (about: "Remove setup hooks")
                    (@arg HOOKS: +takes_value +multiple +required "The list of hooks to remove")
                )
                (@subcommand env =>
                    (about: "Remove environment variables")
                    (@arg VARS: +takes_value +multiple +required "The list of environment variables to remove")
                    (@arg prefix: -p --prefix +takes_value "The prefix to remove the variables from")
                )
            )
            (@subcommand clear =>
                (about: "Clear all values from a config setting")
                (@subcommand setupHooks =>
                    (about: "Clear setup hooks")
                )
                (@subcommand env =>
                    (about: "Clear environment variables")
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
        ("add", Some(args)) => manage_new_game(&config, args),
        ("run", Some(args)) => run_game(&config, args),
        ("hook", Some(args)) => command::hook::dispatch(&config, args),
        ("config", Some(args)) => command::config::dispatch(&mut config, args),
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

fn parse_env_var_args(args: Option<Vec<String>>) -> HashMap<String, String> {
    let args = match args {
        Some(args) => args,
        None => return HashMap::new(),
    };

    let mut env_vars = HashMap::new();

    for arg in args {
        let (name, value) = try_opt_cont!(parse_env_var_arg(arg));
        env_vars.insert(name, value);
    }

    env_vars
}

fn parse_true_false_arg<S>(arg: S) -> bool
where
    S: AsRef<str>,
{
    let arg = arg.as_ref().to_ascii_lowercase();

    match arg.as_ref() {
        "true" | "1" => true,
        _ => false,
    }
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
        force_run_x86: args.is_present("force_run_x86"),
        env_vars: parse_env_var_args(args.values_of_lossy("env_vars")),
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
        env_vars: parse_env_var_args(args.values_of_lossy("env_vars")),
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

mod command {
    use super::*;

    pub mod hook {
        use super::*;

        pub fn dispatch(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
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

    pub mod config {
        use super::*;
        use crate::prefix::Hook;

        pub fn dispatch(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            match args.subcommand() {
                ("set", Some(args)) => handle_set(config, args),
                ("add", Some(args)) => handle_add(config, args),
                ("remove", Some(args)) => handle_remove(config, args),
                ("clear", Some(args)) => handle_clear(config, args),
                _ => unimplemented!(),
            }
        }

        fn handle_set(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            match args.subcommand() {
                ("baseDir", Some(args)) => {
                    let path = PathBuf::from(args.value_of("PATH").unwrap());

                    if !path.exists() {
                        return Err(Error::PathDoesntExist(path.to_string_lossy().into()));
                    }

                    config.base_directory = path;
                    config.save()?;
                }
                ("setupHooks", Some(args)) => {
                    let hooks = args.values_of_lossy("HOOKS").unwrap();
                    verify_hooks_exist(&hooks)?;

                    config.setup_hooks = hooks;
                    config.save()?;
                }
                ("env", Some(args)) => {
                    let env_vars = parse_env_var_args(args.values_of_lossy("VARS"));

                    match args.value_of("prefix") {
                        Some(pfx_name) => {
                            let mut pfx = Prefix::load(pfx_name)?;
                            pfx.env_vars = env_vars;
                            pfx.save()?;
                        }
                        None => {
                            config.global_env_vars = env_vars;
                            config.save()?;
                        }
                    }
                }
                ("forceRunX86", Some(args)) => {
                    let pfx_name = args.value_of("PREFIX").unwrap();
                    let enable = parse_true_false_arg(args.value_of("ENABLE").unwrap());

                    let mut pfx = Prefix::load(pfx_name)?;
                    pfx.force_run_x86 = enable;
                    pfx.save()?;
                }
                _ => unimplemented!(),
            }

            Ok(())
        }

        fn handle_add(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            match args.subcommand() {
                ("setupHooks", Some(args)) => {
                    let hooks = args.values_of_lossy("HOOKS").unwrap();
                    verify_hooks_exist(&hooks)?;

                    for hook in hooks {
                        if config.setup_hooks.contains(&hook) {
                            display::hook(format!(
                                "{} already in setup hooks, skipping",
                                hook.green()
                            ));

                            continue;
                        }

                        config.setup_hooks.push(hook);
                    }

                    config.save()?;
                }
                ("env", Some(args)) => {
                    let env_vars = parse_env_var_args(args.values_of_lossy("VARS"));

                    match args.value_of("prefix") {
                        Some(pfx_name) => {
                            let mut pfx = Prefix::load(pfx_name)?;
                            append_env_vars(env_vars, &mut pfx.env_vars);
                            pfx.save()?;
                        }
                        None => {
                            append_env_vars(env_vars, &mut config.global_env_vars);
                            config.save()?;
                        }
                    }
                }
                _ => unimplemented!(),
            }

            Ok(())
        }

        fn handle_remove(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            match args.subcommand() {
                ("setupHooks", Some(args)) => {
                    let hooks = args.values_of_lossy("HOOKS").unwrap();

                    config.setup_hooks.retain(|setup_hook| {
                        for hook in &hooks {
                            if *hook == *setup_hook {
                                return false;
                            }
                        }

                        true
                    });

                    config.save()?;
                }
                ("env", Some(args)) => {
                    let env_vars = args.values_of_lossy("VARS").unwrap();
                    let remove_vars = |dest: &mut HashMap<String, String>| {
                        for var in env_vars {
                            dest.remove(&var);
                        }
                    };

                    match args.value_of("prefix") {
                        Some(pfx_name) => {
                            let mut pfx = Prefix::load(pfx_name)?;
                            remove_vars(&mut pfx.env_vars);
                            pfx.save()?;
                        }
                        None => {
                            remove_vars(&mut config.global_env_vars);
                            config.save()?;
                        }
                    }
                }
                _ => unimplemented!(),
            }

            Ok(())
        }

        fn handle_clear(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            match args.subcommand() {
                ("setupHooks", Some(_)) => {
                    config.setup_hooks.clear();
                    config.save()?;
                }
                ("env", Some(args)) => match args.value_of("prefix") {
                    Some(pfx_name) => {
                        let mut pfx = Prefix::load(pfx_name)?;
                        pfx.env_vars.clear();
                        pfx.save()?;
                    }
                    None => {
                        config.global_env_vars.clear();
                        config.save()?;
                    }
                },
                _ => unimplemented!(),
            }

            Ok(())
        }

        fn append_env_vars(vars: HashMap<String, String>, dest: &mut HashMap<String, String>) {
            for (name, value) in vars {
                if dest.contains_key(&name) {
                    display::info(format!(
                        "env {} already exists, replacing value",
                        name.blue()
                    ));
                }

                dest.insert(name, value);
            }
        }

        fn verify_hooks_exist(hooks: &[String]) -> Result<(), Error> {
            for hook in hooks {
                if !Hook::get_path(hook)?.exists() {
                    return Err(Error::HookNotFound(hook.clone()));
                }
            }

            Ok(())
        }
    }
}
