pub mod add;

use crate::tui::backend::UIBackend;
use anyhow::Result;
use termion::event::Key;
use tui::layout::Rect;
use tui::terminal::Frame;

pub trait Panel<B: UIBackend> {
    fn tick(&mut self) -> Result<()>;
    fn process_key(&mut self, key: Key) -> Result<()>;

    fn draw(&mut self, rect: Rect, frame: &mut Frame<B::Backend>);
}
