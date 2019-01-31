use crate::display;
use crate::error::CommandError;
use crate::prefix::Prefix;
use colored::Colorize;

pub fn run(args: &clap::ArgMatches) -> Result<(), CommandError> {
    match args.value_of("prefix") {
        Some(pfx_name) => {
            let pfx = Prefix::load(pfx_name)?;
            show_prefix(&pfx);
        }
        None => {
            for pfx in Prefix::load_all()? {
                show_prefix(&pfx);
                println!();
            }
        }
    }

    Ok(())
}

fn show_prefix(pfx: &Prefix) {
    display::prfx(format!("{} data:", pfx.name.blue()));

    for name in pfx.saved_execs.keys() {
        display::exec(format!("{}", name.magenta()));
    }

    for (name, value) in &pfx.env_vars {
        display::env(format!("{} = {}", name.magenta(), value.magenta()));
    }

    if pfx.force_run_x86 {
        display::conf(format!("{}", "force 32-bit mode enabled".magenta()));
    }

    if pfx.run_hooks_explicitly {
        display::conf(format!("{}", "hooks must be run manually".magenta()));
    }
}
