pub mod backend;
mod component;

use crate::config::Config;
use crate::err::{Error, Result};
use backend::{UIBackend, UIEvent, UIEvents};
use chrono::Duration;
use component::applications::Applications;
use component::hooks::Hooks;
use component::tabs::{GenericTab, Tab, TabList};
use component::Component;
use generic_array::arr;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use typenum::U2;

pub struct TUI<B: Backend> {
    backend: UIBackend<B>,
    state: State,
    tabs: TabList<B, U2>,
}

impl<B> TUI<B>
where
    B: Backend,
{
    #[inline(always)]
    pub fn init(backend: UIBackend<B>) -> Result<Self> {
        let tabs = arr![GenericTab<B>;
            Tab::new(
                "Applications",
                Box::new(Applications::new()) as Box<dyn Component<B>>,
            ),
            Tab::new("Hooks", Box::new(Hooks::new()) as Box<dyn Component<B>>),
        ];

        Ok(Self {
            backend,
            state: State::init()?,
            tabs: TabList::new("View", tabs),
        })
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

    fn tick(&mut self) {
        if let LogResult::Err(_, _) = self.tabs.tick(&mut self.state) {
            unimplemented!()
        }
    }

    /// Process a key input for all UI components.
    ///
    /// Returns true if the program should exit.
    fn process_key(&mut self, key: Key) -> bool {
        if let Key::Char('q') = key {
            return true;
        }

        if let LogResult::Err(_, _) = self.tabs.process_key(key, &mut self.state) {
            unimplemented!()
        }

        false
    }

    fn draw_internal(&mut self) -> Result<()> {
        // We need to remove the mutable borrow on self so we can call other mutable methods on it during our draw call.
        // This *should* be completely safe as none of the methods we need to call can mutate our backend.
        let term: *mut _ = &mut self.backend.terminal;
        let term: &mut _ = unsafe { &mut *term };

        term.draw(|mut frame| {
            let tab_splitter = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
                .split(frame.size());

            self.tabs.content_draw_rect = tab_splitter[1];
            self.tabs.draw(&self.state, tab_splitter[0], &mut frame);
        })?;

        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.draw_internal()?;

        self.tabs.after_draw(&mut self.backend);

        Ok(())
    }
}

pub struct State {
    config: Config,
}

impl State {
    fn init() -> Result<Self> {
        let config = Config::load_or_create()?;

        Ok(Self { config })
    }
}

pub enum LogResult {
    Ok,
    Err(String, Error),
}
