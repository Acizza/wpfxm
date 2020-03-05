use super::{Backend, Component, Frame, Rect};
use crate::tui::State;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};

pub struct Hooks {}

impl Hooks {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}

impl<B> Component<B> for Hooks
where
    B: Backend,
{
    fn draw(&mut self, _: &State, rect: Rect, frame: &mut Frame<B>) {
        let horiz_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(rect);

        let left_side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(1)].as_ref())
            .split(horiz_layout[0]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Hooks"))
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">")
            .render(frame, left_side_layout[0]);

        Paragraph::new(
            [Text::styled(
                "Press enter to install",
                Style::default().fg(Color::DarkGray),
            )]
            .iter(),
        )
        .alignment(Alignment::Center)
        .render(frame, left_side_layout[1]);

        let right_side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(1)].as_ref())
            .split(horiz_layout[1]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Prefix"))
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">")
            .render(frame, right_side_layout[0]);

        Paragraph::new(
            [Text::styled(
                "Press u to uninstall",
                Style::default().fg(Color::DarkGray),
            )]
            .iter(),
        )
        .alignment(Alignment::Center)
        .render(frame, right_side_layout[1]);
    }
}
