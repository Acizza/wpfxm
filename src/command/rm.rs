use crate::config::Config;
use crate::display::{self, ErrorSeverity};
use crate::error::CommandError;
use crate::input::{self, Answer};
use crate::prefix::{self, Prefix};
use std::fs;

pub fn run(config: &mut Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    let pfx_name = args.value_of("PREFIX").unwrap();
    let pfx_data_path = Prefix::save_data_path(&pfx_name)?;

    let data_only = args.is_present("data_only");

    if data_only && !pfx_data_path.exists() {
        return Err(CommandError::PrefixDataDoesNotExist);
    }

    let pfx_only = args.is_present("pfx_only");
    let pfx_path = prefix::get_path(config, &pfx_name);

    if pfx_only && !pfx_path.exists() {
        return Err(CommandError::PrefixDoesNotExist);
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
                CommandError::FailedToRemovePath(err, "data file"),
            );
        }
    }

    if !data_only {
        if let Err(err) = fs::remove_dir_all(pfx_path) {
            display::error(
                ErrorSeverity::Warning,
                CommandError::FailedToRemovePath(err, "prefix"),
            );
        }
    }

    config.abs_prefix_paths.remove(pfx_name);
    config.save()?;

    Ok(())
}
