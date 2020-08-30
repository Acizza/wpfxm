mod config;
mod err;
mod prefix;
mod tui;
mod util;

use crate::tui::backend::UIBackend;
use anyhow::Result;

fn main() -> Result<()> {
    let backend = UIBackend::init()?;

    Ok(())
}
