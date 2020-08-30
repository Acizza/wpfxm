pub mod backend;
pub mod panel;

use anyhow::{Context, Result};
use backend::{UIBackend, UIEvent, UIEvents};
use chrono::Duration;
use panel::Panel;
use termion::event::Key;

pub struct PanelHandler<B, P>
where
    B: UIBackend,
    P: Panel<B>,
{
    backend: B,
    panel: P,
}

impl<B, P> PanelHandler<B, P>
where
    B: UIBackend,
    P: Panel<B>,
{
    pub fn init(panel: P) -> Result<Self> {
        let backend = B::init()?;
        Ok(Self { backend, panel })
    }

    pub fn run(mut self) -> Result<()> {
        let events = UIEvents::new(Duration::seconds(1));

        loop {
            self.draw()?;

            match events.next()? {
                UIEvent::Input(key) => {
                    if self.process_key(key)? {
                        self.exit().ok();
                        break Ok(());
                    }
                }
                UIEvent::Tick => self.tick()?,
            }
        }
    }

    #[inline(always)]
    pub fn exit(mut self) -> Result<()> {
        self.backend.clear()
    }

    #[inline(always)]
    fn tick(&mut self) -> Result<()> {
        self.panel.tick()
    }

    /// Process a key input for all UI components.
    ///
    /// Returns true if the program should exit.
    fn process_key(&mut self, key: Key) -> Result<bool> {
        if let Key::Char('q') = key {
            return Ok(true);
        }

        self.panel
            .process_key(key)
            .context("key processing for panel failed")?;

        Ok(false)
    }

    fn draw(&mut self) -> Result<()> {
        // We need to remove the mutable borrow on self so we can call other mutable methods on it during our draw call.
        // This *should* be completely safe as none of the methods we need to call can mutate our backend.
        let term: *mut _ = self.backend.terminal_mut();
        let term: &mut _ = unsafe { &mut *term };

        term.draw(|mut frame| {
            self.panel.draw(frame.size(), &mut frame);
        })?;

        Ok(())
    }
}
