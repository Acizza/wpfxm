pub mod backend;

use crate::err::Result;
use backend::{UIBackend, UIEvent, UIEvents};
use chrono::Duration;
use termion::event::Key;
use tui::backend::Backend;

pub struct TUI<B: Backend> {
    backend: UIBackend<B>,
}

impl<B> TUI<B>
where
    B: Backend,
{
    #[inline(always)]
    pub fn new(backend: UIBackend<B>) -> Self {
        Self { backend }
    }

    pub fn run(mut self) -> Result<()> {
        let events = UIEvents::new(Duration::seconds(1));

        loop {
            self.draw()?;

            match events.next()? {
                UIEvent::Input(key) => {
                    if self.process_key(key) {
                        self.exit().ok();
                        break Ok(());
                    }
                }
                UIEvent::Tick => self.tick(),
            }
        }
    }

    pub fn exit(mut self) -> Result<()> {
        self.backend.terminal.clear().map_err(Into::into)
    }

    fn tick(&mut self) {}

    /// Process a key input for all UI components.
    ///
    /// Returns true if the program should exit.
    fn process_key(&mut self, key: Key) -> bool {
        if let Key::Char('q') = key {
            return true;
        }

        false
    }

    fn draw(&mut self) -> Result<()> {
        Ok(())
    }
}
