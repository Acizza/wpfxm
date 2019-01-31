use crate::config::Config;
use crate::display;
use crate::error::CommandError;
use crate::prefix::Prefix;
use colored::Colorize;
use std::borrow::Cow;

pub fn run(config: &mut Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    clean_config(config)?;

    match args.value_of("prefix") {
        Some(pfx_name) => {
            let mut pfx = Prefix::load(pfx_name)?;
            clean_prefix(config, &mut pfx)?;
        }
        None => {
            for mut pfx in Prefix::load_all()? {
                clean_prefix(config, &mut pfx)?;
            }
        }
    }

    Ok(())
}

fn clean_config(config: &mut Config) -> Result<(), CommandError> {
    config.abs_prefix_paths.retain(|pfx_name, path| {
        let exists = path.exists();

        if !exists {
            display::info(format!(
                "removing absolute prefix {} at path {}",
                pfx_name.blue(),
                path.to_string_lossy().blue()
            ));
        }

        exists
    });

    config.save()?;
    Ok(())
}

fn clean_prefix(config: &Config, pfx: &mut Prefix) -> Result<(), CommandError> {
    let pfx_name = Cow::Borrowed(&pfx.name);
    let pfx_path = pfx.get_prefix_path(config);

    pfx.saved_execs.retain(|name, path| {
        let exists = pfx_path.join(path).exists();

        if !exists {
            display::info(format!(
                "removing application {} in prefix {}",
                name.blue(),
                pfx_name.blue()
            ));
        }

        exists
    });

    pfx.save()?;
    Ok(())
}
