use super::{Backend, Component, Frame, LogResult, Rect};
use crate::tui::State;
use termion::event::Key;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};

pub struct Applications;

impl Applications {
    #[inline(always)]
    pub fn init(state: &mut State) -> Self {
        if let Some(pfx) = state.prefixes.selected_mut() {
            pfx.populate_applications();
        }

        Self {}
    }

    fn draw_prefix_list<B>(state: &State, rect: Rect, frame: &mut Frame<B>)
    where
        B: Backend,
    {
        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Prefix"))
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">")
            .items(state.prefixes.items())
            .select(Some(state.prefixes.index()))
            .render(frame, rect);
    }

    fn draw_info_panel<B>(state: &State, rect: Rect, frame: &mut Frame<B>)
    where
        B: Backend,
    {
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
                Text::styled(format!("{}\n", $value), Style::default().modifier(Modifier::ITALIC))
            };
        }

        let info_block = Block::default().title("Info").borders(Borders::ALL);

        match state.prefixes.selected() {
            Some(pfx) => {
                let stats = create_stat_list!(
                    "Applications" => pfx.applications.len(),
                    "Arch" => pfx.arch,
                    "Wine Version" => "5.3"
                );

                Paragraph::new(stats.iter())
                    .block(info_block)
                    .render(frame, rect);
            }
            None => Paragraph::new([].iter())
                .block(info_block)
                .render(frame, rect),
        }
    }

    fn draw_applications_list<B>(state: &State, rect: Rect, frame: &mut Frame<B>)
    where
        B: Backend,
    {
        let applications = state
            .prefixes
            .selected()
            .map(|pfx| pfx.applications.as_slice())
            .unwrap_or_else(|| &[]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Applications"))
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">")
            .items(applications)
            .render(frame, rect);
    }

    fn draw_hint_text<B>(rect: Rect, frame: &mut Frame<B>)
    where
        B: Backend,
    {
        let hint_text_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(rect);

        Paragraph::new(
            [Text::styled(
                "Enter to select prefix / run selected",
                Style::default().fg(Color::DarkGray),
            )]
            .iter(),
        )
        .alignment(Alignment::Center)
        .render(frame, hint_text_layout[0]);

        Paragraph::new(
            [Text::styled(
                "R to run input",
                Style::default().fg(Color::DarkGray),
            )]
            .iter(),
        )
        .alignment(Alignment::Center)
        .render(frame, hint_text_layout[1]);
    }
}

impl<B> Component<B> for Applications
where
    B: Backend,
{
    fn process_key(&mut self, key: Key, state: &mut State) -> LogResult {
        match key {
            Key::Up | Key::Down => {
                match key {
                    Key::Up => state.prefixes.decrement(),
                    Key::Down => state.prefixes.increment(),
                    _ => unreachable!(),
                }

                if let Some(pfx) = state.prefixes.selected_mut() {
                    pfx.populate_applications();
                }

                LogResult::Ok
            }
            _ => LogResult::Ok,
        }
    }

    fn draw(&mut self, state: &State, rect: Rect, frame: &mut Frame<B>) {
        let vert_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(1)].as_ref())
            .split(rect);

        let horiz_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(vert_layout[0]);

        let prefix_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(horiz_layout[0]);

        Self::draw_prefix_list(state, prefix_layout[0], frame);
        Self::draw_info_panel(state, prefix_layout[1], frame);
        Self::draw_applications_list(state, horiz_layout[1], frame);
        Self::draw_hint_text(vert_layout[1], frame);
    }
}
