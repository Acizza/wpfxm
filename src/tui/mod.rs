pub mod backend;
mod panel;

use anyhow::{Context, Result};
use backend::{UIBackend, UIEvent, UIEvents};
use chrono::Duration;
use panel::Panel;
use termion::event::Key;
use tui::backend::Backend;

pub struct PanelHandler<B, P>
where
    B: Backend,
    P: Panel<B>,
{
    backend: UIBackend<B>,
    panel: P,
}

impl<B, P> PanelHandler<B, P>
where
    B: Backend,
    P: Panel<B>,
{
    pub fn init(backend: UIBackend<B>) -> Result<Self> {
        let panel = P::init().context("failed to init panel")?;
        Ok(Self { backend, panel })
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
                UIEvent::Tick => self.tick()?,
            }
        }
    }

    pub fn exit(mut self) -> Result<()> {
        self.backend.terminal.clear().map_err(Into::into)
    }

    #[inline(always)]
    fn tick(&mut self) -> Result<()> {
        self.panel.tick()
    }

    /// Process a key input for all UI components.
    ///
    /// Returns true if the program should exit.
    fn process_key(&mut self, key: Key) -> bool {
        if let Key::Char('q') = key {
            return true;
        }

        self.panel
            .process_key(key)
            .context("key processing for panel failed");

        false
    }

    fn draw(&mut self) -> Result<()> {
        // We need to remove the mutable borrow on self so we can call other mutable methods on it during our draw call.
        // This *should* be completely safe as none of the methods we need to call can mutate our backend.
        let term: *mut _ = &mut self.backend.terminal;
        let term: &mut _ = unsafe { &mut *term };

        term.draw(|mut frame| {
            self.panel.draw(frame.size(), &mut frame);
        })?;

        Ok(())
    }
}
