use crate::config::Config;
use crate::display;
use crate::error::CommandError;
use crate::prefix::hook::Hook;
use crate::prefix::Prefix;
use colored::Colorize;
use hashbrown::HashMap;
use std::path::PathBuf;

pub fn run(config: &mut Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    match args.subcommand() {
        ("set", Some(args)) => handle_set(config, args),
        ("rm", Some(args)) => handle_rm(config, args),
        _ => unimplemented!(),
    }
}

fn handle_set(config: &mut Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    match args.subcommand() {
        ("baseDir", Some(args)) => {
            let path = PathBuf::from(args.value_of("PATH").unwrap());

            if !path.exists() {
                return Err(CommandError::PathDoesntExist(path.to_string_lossy().into()));
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
            let env_vars = super::parse_env_var_args(args.values_of_lossy("VARS"));
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
            let enable = super::parse_true_false_arg(args.value_of("ENABLE").unwrap());

            let mut pfx = Prefix::load(pfx_name)?;
            pfx.force_run_x86 = enable;
            pfx.save()?;
        }
        ("explicitHooks", Some(args)) => {
            let pfx_name = args.value_of("PREFIX").unwrap();
            let enable = super::parse_true_false_arg(args.value_of("ENABLE").unwrap());

            let mut pfx = Prefix::load(pfx_name)?;
            pfx.run_hooks_explicitly = enable;
            pfx.save()?;
        }
        _ => unimplemented!(),
    }

    Ok(())
}

fn handle_rm(config: &mut Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
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

fn verify_hooks_exist(hooks: &[String]) -> Result<(), CommandError> {
    for hook in hooks {
        if !Hook::get_path(hook)?.exists() {
            return Err(CommandError::HookNotFound(hook.clone()));
        }
    }

    Ok(())
}
