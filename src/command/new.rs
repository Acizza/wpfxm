use crate::config::Config;
use crate::display;
use crate::error::CommandError;
use crate::prefix::{self, LaunchOptions, Prefix, PrefixArch};
use colored::Colorize;
use std::collections::HashMap;

pub fn run(config: &mut Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
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

        let mut process = pfx
            .launch_non_wine_process(config, &process_name, opts)
            .map_err(|err| CommandError::FailedToRunProcess(err, process_name.clone()))?;

        if let Some(exec_name) = args.value_of("add") {
            process.wait().ok();
            super::add::manage_new_exec(config, pfx_name, exec_name)?;
        }
    }

    Ok(())
}

fn load_or_create_pfx<S>(
    config: &Config,
    args: &clap::ArgMatches,
    pfx_name: S,
) -> Result<Prefix, CommandError>
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
        return Err(CommandError::PrefixAlreadyManaged(pfx_name.into()));
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
            env_vars: super::parse_env_var_args(args.values_of_lossy("env_vars")),
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
