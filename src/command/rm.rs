use crate::config::Config;
use crate::display::{self, ErrorSeverity};
use crate::error::CommandError;
use crate::input::{self, Answer};
use crate::prefix::{self, Prefix};
use std::fs;

pub fn run(config: &mut Config, args: &clap::ArgMatches) -> Result<(), CommandError> {
    let pfx_name = args.value_of("PREFIX").unwrap();
    let pfx_data_path = Prefix::save_data_path(&pfx_name)?;
    let pfx_path = prefix::get_path(config, &pfx_name);

    display::input("prefix paths:");
    display::input(format!("save data: {}", pfx_data_path.display()));
    display::input(format!("prefix: {}", pfx_path.display()));

    display::input("are these paths correct? (Y/n)");

    if input::read_yn(Answer::Yes)? == Answer::No {
        return Ok(());
    }

    // Remove prefix
    if !args.is_present("data_only") && pfx_path.exists() {
        if let Err(err) = fs::remove_dir_all(pfx_path) {
            display::error(
                ErrorSeverity::Warning,
                CommandError::FailedToRemovePath(err, "prefix"),
            );
        }
    }

    // Remove prefix save data
    if !args.is_present("pfx_only") && pfx_data_path.exists() {
        if let Err(err) = fs::remove_file(pfx_data_path) {
            display::error(
                ErrorSeverity::Warning,
                CommandError::FailedToRemovePath(err, "save data"),
            );
        }
    }

    config.abs_prefix_paths.remove(pfx_name);
    config.save()?;

    Ok(())
}
