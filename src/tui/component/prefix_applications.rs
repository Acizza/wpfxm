use super::{Backend, Component, Frame, Rect};
use crate::tui::State;
use tui::widgets::{Block, Borders, Widget};

pub struct PrefixApplications {}

impl PrefixApplications {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}

impl<B> Component<B> for PrefixApplications
where
    B: Backend,
{
    fn draw(&mut self, _: &State, rect: Rect, frame: &mut Frame<B>) {
        Block::default().borders(Borders::ALL).render(frame, rect);
    }
}
