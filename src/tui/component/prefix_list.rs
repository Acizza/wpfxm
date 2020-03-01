use super::{Backend, Component, Frame, Rect};
use crate::tui::State;
use tui::widgets::{Block, Borders, Widget};

pub struct PrefixList {}

impl PrefixList {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}

impl<B> Component<B> for PrefixList
where
    B: Backend,
{
    fn draw(&mut self, _: &State, rect: Rect, frame: &mut Frame<B>) {
        Block::default().borders(Borders::ALL).render(frame, rect);
    }
}
