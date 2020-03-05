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

pub struct WrappingSelection<T> {
    items: Vec<T>,
    selected: usize,
}

impl<T> WrappingSelection<T> {
    #[inline(always)]
    pub fn new<I>(items: I) -> Self
    where
        I: Into<Vec<T>>,
    {
        Self {
            items: items.into(),
            selected: 0,
        }
    }

    #[inline(always)]
    pub fn index(&self) -> usize {
        self.selected
    }

    #[inline(always)]
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    #[inline(always)]
    pub fn increment(&mut self) {
        let next = self.selected + 1;
        self.selected = if next >= self.items.len() { 0 } else { next };
    }

    #[inline(always)]
    pub fn decrement(&mut self) {
        self.selected = if self.selected == 0 {
            self.items.len().saturating_sub(1)
        } else {
            self.selected - 1
        }
    }
}
