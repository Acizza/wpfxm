use crate::config::Config;
use crate::error::CommandError;
use crate::prefix::hook;
use crate::prefix::Prefix;

pub fn run(config: &Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    match args.subcommand() {
        ("run", Some(args)) => run_hooks(config, args),
        _ => unreachable!(),
    }
}

fn run_hooks(config: &Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    let hooks = args.values_of_lossy("HOOKS").unwrap();

    match args.value_of("prefix") {
        Some(pfx_name) => {
            let prefix = Prefix::load(pfx_name)?;
            hook::run_list(config, &prefix, &hooks);
        }
        None => {
            let prefixes = Prefix::load_all()?;

            for prefix in prefixes {
                if prefix.run_hooks_explicitly {
                    continue;
                }

                hook::run_list(config, &prefix, &hooks);
            }
        }
    }

    Ok(())
}
