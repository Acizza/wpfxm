use super::Panel;
use crate::config::Config;
use crate::prefix::Prefix;
use crate::tui::backend::UIBackend;
use anyhow::{Context, Result};
use termion::event::Key;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::text::Span;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

pub struct AddPanel {
    list_state: ListState,
    prefix: Prefix,
}

impl AddPanel {
    pub fn init(pfx_name: String) -> Result<Self>
    where
        Self: Sized,
    {
        let config = Config::load_or_create()?;
        let pfx_path = config.prefix_path.join(&pfx_name);

        let mut prefix = Prefix::from_path(pfx_path, pfx_name).context("failed to load prefix")?;
        prefix.populate_applications();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(Self { list_state, prefix })
    }

    fn draw_list<B: UIBackend>(&mut self, rect: Rect, frame: &mut Frame<B::Backend>) {
        let items = self
            .prefix
            .found_applications
            .iter()
            .map(|app| app.to_string_lossy())
            .map(Span::raw)
            .map(ListItem::new)
            .collect::<Vec<_>>();

        let app_list = List::new(items)
            .block(
                Block::default()
                    .title("Select an application to add")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">");

        frame.render_stateful_widget(app_list, rect, &mut self.list_state);
    }

    fn draw_hints<B: UIBackend>(rect: Rect, frame: &mut Frame<B::Backend>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rect);

        let left_text = Span::styled(
            "Navigate with arrow keys",
            Style::default().fg(Color::DarkGray),
        );
        let left_widget = Paragraph::new(left_text).alignment(Alignment::Center);
        frame.render_widget(left_widget, layout[0]);

        let right_text = Span::styled(
            "Enter to add application",
            Style::default().fg(Color::DarkGray),
        );
        let right_widget = Paragraph::new(right_text).alignment(Alignment::Center);
        frame.render_widget(right_widget, layout[1]);
    }
}

impl<B: UIBackend> Panel<B> for AddPanel {
    fn tick(&mut self) -> Result<()> {
        Ok(())
    }

    fn process_key(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Up | Key::Down => {
                let selected = self.list_state.selected().unwrap_or(0);

                let new_index = match key {
                    Key::Up => {
                        if selected == 0 {
                            self.prefix.found_applications.len() - 1
                        } else {
                            selected - 1
                        }
                    }
                    Key::Down => (selected + 1) % self.prefix.found_applications.len(),
                    _ => unreachable!(),
                };

                self.list_state.select(Some(new_index));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn draw(&mut self, rect: Rect, frame: &mut Frame<B::Backend>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(1)])
            .split(rect);

        self.draw_list::<B>(layout[0], frame);
        Self::draw_hints::<B>(layout[1], frame);
    }
}
