pub mod applications;
pub mod hooks;
pub mod tabs;

use super::{LogResult, State, UIBackend};
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

pub trait Component<B: Backend> {
    fn tick(&mut self, _: &mut State) -> LogResult {
        LogResult::Ok
    }

    fn process_key(&mut self, _: Key, _: &mut State) -> LogResult {
        LogResult::Ok
    }

    fn draw(&mut self, state: &State, rect: Rect, frame: &mut Frame<B>);
    fn after_draw(&mut self, _: &mut UIBackend<B>) {}
}
