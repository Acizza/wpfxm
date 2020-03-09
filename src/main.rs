mod config;
mod err;
mod prefix;
mod tui;
mod util;

use crate::err::Result;
use crate::tui::backend::UIBackend;
use crate::tui::TUI;

fn main() {
    if let Err(err) = run() {
        err::display_error(err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let backend = UIBackend::init()?;
    let tui = TUI::init(backend)?;

    tui.run()?;

    Ok(())
}
