#[macro_use]
mod util;

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
            (@arg arch: -a --arch +takes_value default_value("win64") "The architecture to use for the prefix")
            (@arg env_vars: -e --env +takes_value +multiple "The environment variables to use for the prefix")
            (@arg run: -r --run +takes_value +multiple "The program to run after prefix creation")
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
            (@arg name: -n --name +takes_value "The name of the application to run; can be omitted if there is only one application managed")
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
        ("new", Some(args)) => command::new::create_prefix(&mut config, args),
        ("add", Some(args)) => command::add::manage_new_exec(&config, args),
        ("run", Some(args)) => command::run::run_exec(&config, args),
        ("exec", Some(args)) => command::exec::run_exec_in_pfx(&config, args),
        ("hook", Some(args)) => command::hook::dispatch(&config, args),
        ("rm", Some(args)) => command::rm::remove_prefix(&mut config, args),
        ("cfg", Some(args)) => command::cfg::dispatch(&mut config, args),
        _ => unreachable!(),
    }
}

mod command {
    // Here's what may be a compiler bug!
    // Rust says that this import isn't used, but you'll get errors about
    // functions from this trait not being in scope if it's removed
    #[allow(unused_imports)]
    use colored::Colorize;

    use crate::config::Config;
    use crate::display;
    use crate::error::{Error, PrefixError};
    use crate::input;
    use crate::prefix::{self, LaunchOptions, Prefix, PrefixArch};
    use std::collections::HashMap;
    use std::path::PathBuf;

    pub mod new {
        use super::*;

        pub fn create_prefix(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            let pfx_name = args.value_of("PREFIX").unwrap();

            if let Some(path) = args.value_of("path") {
                config.abs_prefix_paths.insert(pfx_name.into(), path.into());
                config.save()?;
            }

            let pfx = load_or_create_pfx(config, args, pfx_name)?;

            if !args.is_present("explicit_hooks") {
                display::hook("running setup hooks");
                pfx.run_hooks(config, &config.setup_hooks);
            }

            if let Some(mut run_args) = args.values_of_lossy("run") {
                let process_name = run_args.remove(0);
                display::info(format!("running [{}]", process_name.blue()));

                let opts = LaunchOptions {
                    force_run_x86: pfx.force_run_x86,
                    env_vars: pfx.env_vars.clone(),
                    args: run_args,
                };

                pfx.launch_non_wine_process(config, &process_name, opts)
                    .map_err(|err| Error::FailedToRunProcess(err, process_name.clone()))?;
            }

            Ok(())
        }

        fn load_or_create_pfx<S>(
            config: &Config,
            args: &clap::ArgMatches,
            pfx_name: S,
        ) -> Result<Prefix, Error>
        where
            S: AsRef<str>,
        {
            let pfx_name = pfx_name.as_ref();
            let pfx_path = prefix::get_path(config, &pfx_name);
            let pfx_exists = prefix::exists_and_valid(&pfx_path);
            let pfx_data_exists = Prefix::get_data_file(pfx_name)
                .map(|f| f.exists())
                .unwrap_or(false);

            if pfx_exists && pfx_data_exists {
                return Err(Error::PrefixAlreadyManaged(pfx_name.into()));
            }

            let pfx = if pfx_data_exists {
                display::info("using existing prefix save data");
                Prefix::load(pfx_name)?
            } else {
                display::info("creating new prefix save data");

                let arch = if pfx_exists {
                    prefix::detect_arch(&pfx_path)?
                } else {
                    args.value_of("arch")
                        .and_then(PrefixArch::parse)
                        .unwrap_or_default()
                };

                Prefix {
                    name: pfx_name.into(),
                    run_hooks_explicitly: args.is_present("explicit_hooks"),
                    arch,
                    force_run_x86: args.is_present("force_run_x86"),
                    saved_execs: HashMap::new(),
                    env_vars: parse_env_var_args(args.values_of_lossy("env_vars")),
                }
            };

            if !pfx_exists {
                display::info("creating prefix");
                pfx.create(config)?;
            }

            if !pfx_data_exists {
                pfx.save()?;
            }

            Ok(pfx)
        }
    }

    pub mod add {
        use super::*;

        pub fn manage_new_exec(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
            let pfx_name = args.value_of("PREFIX").unwrap();

            if !Prefix::get_data_file(&pfx_name)?.exists() {
                return Err(Error::PrefixNotManaged(pfx_name.into()));
            }

            let mut prefix = Prefix::load(pfx_name)?;

            let exec_name = args.value_of("NAME").unwrap();
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

        fn select_exec(config: &Config, pfx: &Prefix) -> Result<PathBuf, Error> {
            let pfx_path = pfx.get_prefix_path(config);
            let mut found = prefix::scan::unique_executables(&pfx_path, pfx.arch);

            if found.is_empty() {
                return Err(Error::NoExecsDetected(pfx.name.clone()));
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
    }

    pub mod run {
        use super::*;

        pub fn run_exec(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
            let pfx_name = args.value_of("PREFIX").unwrap();

            let prefix = match Prefix::load(pfx_name) {
                Ok(pfx) => pfx,
                Err(PrefixError::FailedToReadConfig(_)) => {
                    return Err(Error::PrefixNotManaged(pfx_name.into()));
                }
                Err(err) => return Err(err.into()),
            };

            // TODO: simplify
            let (exec_name, exec_path) = match prefix.saved_execs.len() {
                0 => return Err(Error::NoSavedExecs),
                1 => {
                    let (name, value) = prefix.saved_execs.iter().next().unwrap();
                    (name.clone(), value.clone())
                }
                _ => {
                    let exec_name = args.value_of("name").ok_or(Error::NameNeededToRunExec)?;

                    if !prefix.saved_execs.contains_key(exec_name) {
                        return Err(Error::ExecNotManaged(exec_name.into()))?;
                    }

                    let value = prefix.saved_execs[exec_name].clone();
                    (exec_name.into(), value)
                }
            };

            display::info(format!("running {}", exec_name.blue()));

            let launch_opts = LaunchOptions {
                env_vars: parse_env_var_args(args.values_of_lossy("env_vars")),
                force_run_x86: prefix.force_run_x86 || args.is_present("force_run_x86"),
                args: Vec::new(),
            };

            if let Err(err) = prefix.launch_prefix_process(config, &exec_path, launch_opts) {
                return Err(Error::FailedToRunProcess(
                    err,
                    exec_path.to_string_lossy().to_string(),
                ));
            }

            Ok(())
        }
    }

    pub mod exec {
        use super::*;

        pub fn run_exec_in_pfx(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
            let pfx_name = args.value_of("PREFIX").unwrap();

            if !prefix::get_path(config, &pfx_name).exists() {
                return Err(Error::PrefixDoesNotExist);
            }

            let pfx = match Prefix::load(pfx_name) {
                Ok(pfx) => pfx,
                Err(PrefixError::FailedToReadConfig(_)) => {
                    return Err(Error::PrefixNotManaged(pfx_name.into()));
                }
                Err(err) => return Err(err.into()),
            };

            let mut run_args = args.values_of_lossy("ARGS").unwrap();
            let exe_name = run_args.remove(0);

            display::info(format!("running [{}]", exe_name.blue()));

            let opts = LaunchOptions {
                force_run_x86: false, // This doesn't matter when running a non-Wine process
                env_vars: parse_env_var_args(args.values_of_lossy("env_vars")),
                args: run_args,
            };

            pfx.launch_non_wine_process(config, &exe_name, opts)
                .map_err(|err| Error::FailedToRunProcess(err, exe_name))?;

            Ok(())
        }
    }

    pub mod hook {
        use super::*;

        pub fn dispatch(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
            match args.subcommand() {
                ("run", Some(args)) => run_hooks(config, args),
                _ => unreachable!(),
            }
        }

        fn run_hooks(config: &Config, args: &clap::ArgMatches) -> Result<(), Error> {
            let hooks = args.values_of_lossy("HOOKS").unwrap();

            match args.value_of("prefix") {
                Some(pfx_name) => {
                    let prefix = Prefix::load(pfx_name)?;
                    prefix.run_hooks(config, &hooks);
                }
                None => {
                    let prefixes = Prefix::load_all()?;

                    for prefix in prefixes {
                        if prefix.run_hooks_explicitly {
                            continue;
                        }

                        prefix.run_hooks(config, &hooks);
                    }
                }
            }

            Ok(())
        }
    }

    pub mod rm {
        use super::*;
        use display::ErrorSeverity;
        use input::Answer;
        use std::fs;

        pub fn remove_prefix(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            let pfx_name = args.value_of("PREFIX").unwrap();
            let pfx_data_path = Prefix::get_data_file(&pfx_name)?;

            let data_only = args.is_present("data_only");

            if data_only && !pfx_data_path.exists() {
                return Err(Error::PrefixDataDoesNotExist);
            }

            let pfx_only = args.is_present("pfx_only");
            let pfx_path = prefix::get_path(config, &pfx_name);

            if pfx_only && !pfx_path.exists() {
                return Err(Error::PrefixDoesNotExist);
            }

            display::input("paths to be deleted:");

            if !pfx_only {
                display::input(format!("save data: {}", pfx_data_path.display()));
            }

            if !data_only {
                display::input(format!("prefix: {}", pfx_path.display()));
            }

            display::input("is this okay? (Y/n)");

            if input::read_yn(Answer::Yes)? == Answer::No {
                return Ok(());
            }

            if !pfx_only {
                if let Err(err) = fs::remove_file(pfx_data_path) {
                    display::error(
                        ErrorSeverity::Warning,
                        Error::FailedToRemovePath(err, "data file"),
                    );
                }
            }

            if !data_only {
                if let Err(err) = fs::remove_dir_all(pfx_path) {
                    display::error(
                        ErrorSeverity::Warning,
                        Error::FailedToRemovePath(err, "prefix"),
                    );
                }
            }

            config.abs_prefix_paths.remove(pfx_name);
            config.save()?;

            Ok(())
        }
    }

    pub mod cfg {
        use super::*;
        use crate::prefix::Hook;

        pub fn dispatch(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            match args.subcommand() {
                ("set", Some(args)) => handle_set(config, args),
                ("rm", Some(args)) => handle_rm(config, args),
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

                    if args.is_present("append") {
                        append_hooks(hooks, &mut config.setup_hooks);
                    } else {
                        config.setup_hooks = hooks;
                    }

                    config.save()?;
                }
                ("env", Some(args)) => {
                    let env_vars = parse_env_var_args(args.values_of_lossy("VARS"));
                    let append = args.is_present("append");

                    let mutate_env_vars = |dest: &mut HashMap<String, String>| {
                        if append {
                            append_env_vars(env_vars, dest);
                            return;
                        }

                        *dest = env_vars;
                    };

                    match args.value_of("prefix") {
                        Some(pfx_name) => {
                            let mut pfx = Prefix::load(pfx_name)?;
                            mutate_env_vars(&mut pfx.env_vars);
                            pfx.save()?;
                        }
                        None => {
                            mutate_env_vars(&mut config.global_env_vars);
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
                ("explicitHooks", Some(args)) => {
                    let pfx_name = args.value_of("PREFIX").unwrap();
                    let enable = parse_true_false_arg(args.value_of("ENABLE").unwrap());

                    let mut pfx = Prefix::load(pfx_name)?;
                    pfx.run_hooks_explicitly = enable;
                    pfx.save()?;
                }
                _ => unimplemented!(),
            }

            Ok(())
        }

        fn handle_rm(config: &mut Config, args: &clap::ArgMatches) -> Result<(), Error> {
            match args.subcommand() {
                ("setupHooks", Some(_)) => {
                    let hooks = args.values_of_lossy("hooks").unwrap_or_else(Vec::new);

                    if hooks.is_empty() {
                        config.setup_hooks.clear();
                    } else {
                        config.setup_hooks.retain(|setup_hook| {
                            for hook in &hooks {
                                if *hook == *setup_hook {
                                    return false;
                                }
                            }

                            true
                        });
                    }

                    config.save()?;
                }
                ("env", Some(args)) => {
                    let env_vars = args.values_of_lossy("vars").unwrap_or_else(Vec::new);

                    let remove_vars = |dest: &mut HashMap<String, String>| {
                        if env_vars.is_empty() {
                            dest.clear();
                            return;
                        }

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

        fn append_hooks(hooks: Vec<String>, dest: &mut Vec<String>) {
            for hook in hooks {
                if dest.contains(&hook) {
                    display::hook(format!("{} already exists, skipping", hook.green()));
                    continue;
                }

                dest.push(hook);
            }
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
}
