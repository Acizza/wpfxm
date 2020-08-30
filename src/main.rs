mod config;
mod err;
mod prefix;
mod tui;
mod util;

use crate::tui::backend::DefaultBackend;
use crate::tui::panel::add::AddPanel;
use crate::tui::PanelHandler;
use anyhow::Result;
use gumdrop::Options;

#[derive(Options)]
struct CmdArgs {
    #[options(free)]
    free: Vec<String>,
    help: bool,
    #[options(help = "add an application to the given prefix", meta = "PREFIX")]
    add: Option<String>,
}

fn main() -> Result<()> {
    let args = CmdArgs::parse_args_default_or_exit();

    if let Some(pfx_name) = args.add {
        let tui = PanelHandler::<DefaultBackend, AddPanel>::init()?;
        tui.run()?;
    }

    Ok(())
}
