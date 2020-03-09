pub mod backend;
mod component;

use crate::config::Config;
use crate::err::{Error, Result};
use crate::prefix::Prefix;
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
    prefixes: WrappingSelection<Prefix>,
    config: Config,
}

impl State {
    fn init() -> Result<Self> {
        let config = Config::load_or_create()?;
        let prefixes = Prefix::all_from_dir(&config.prefix_path)?.into();

        Ok(Self { prefixes, config })
    }
}

pub enum LogResult {
    Ok,
    Err(String, Error),
}

pub struct WrappingSelection<T> {
    items: Vec<T>,
    selected: usize,
}

impl<T> WrappingSelection<T> {
    #[inline(always)]
    pub fn new<I>(items: I) -> Self
    where
        I: Into<Vec<T>>,
    {
        Self {
            items: items.into(),
            selected: 0,
        }
    }

    #[inline(always)]
    pub fn index(&self) -> usize {
        self.selected
    }

    #[inline(always)]
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    #[inline(always)]
    pub fn increment(&mut self) {
        let next = self.selected + 1;
        self.selected = if next >= self.items.len() { 0 } else { next };
    }

    #[inline(always)]
    pub fn decrement(&mut self) {
        self.selected = if self.selected == 0 {
            self.items.len().saturating_sub(1)
        } else {
            self.selected - 1
        }
    }
}

impl<T> From<Vec<T>> for WrappingSelection<T> {
    fn from(items: Vec<T>) -> Self {
        Self::new(items)
    }
}
