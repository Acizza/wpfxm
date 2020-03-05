use super::{Backend, Component, Frame, LogResult, Rect, WrappingSelection};
use crate::tui::State;
use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};

pub struct Applications {
    prefixes: WrappingSelection<&'static str>,
}

impl Applications {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            prefixes: WrappingSelection::new(vec!["Test 1", "Test 2", "Test 3"]),
        }
    }
}

macro_rules! create_stat_list {
    ($($header:expr => $value:expr),+) => {
        [$(
            create_stat_list!(h $header),
            create_stat_list!(v $value),
        )+]
    };

    (h $header:expr) => {
        Text::styled(concat!($header, ": "), Style::default().modifier(Modifier::BOLD))
    };

    (v $value:expr) => {
        Text::styled(concat!($value, "\n"), Style::default().modifier(Modifier::ITALIC))
    };
}

impl<B> Component<B> for Applications
where
    B: Backend,
{
    fn process_key(&mut self, key: Key, _: &mut State) -> LogResult {
        match key {
            Key::Up => {
                self.prefixes.decrement();
                LogResult::Ok
            }
            Key::Down => {
                self.prefixes.increment();
                LogResult::Ok
            }
            _ => LogResult::Ok,
        }
    }

    fn draw(&mut self, _: &State, rect: Rect, frame: &mut Frame<B>) {
        let horiz_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(rect);

        let prefix_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(horiz_layout[0]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Prefix"))
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">")
            .items(self.prefixes.items())
            .select(Some(self.prefixes.index()))
            .render(frame, prefix_layout[0]);

        let info_items = create_stat_list!(
            "Applications" => "42",
            "Arch" => "x86-64",
            "Wine Version" => "5.3"
        );

        Paragraph::new(info_items.iter())
            .block(Block::default().title("Info").borders(Borders::ALL))
            .render(frame, prefix_layout[1]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Applications"))
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">")
            .render(frame, horiz_layout[1]);
    }
}
