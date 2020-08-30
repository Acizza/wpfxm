use anyhow::{Context, Result};
use chrono::Duration;
use std::io;
use std::sync::mpsc;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::{self, Backend};
use tui::terminal::Terminal;

pub struct UIBackend<B>
where
    B: Backend,
{
    pub terminal: Terminal<B>,
}

pub type TermionBackend = backend::TermionBackend<RawTerminal<io::Stdout>>;

impl UIBackend<TermionBackend> {
    pub fn init() -> Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;
        terminal.hide_cursor()?;

        Ok(Self { terminal })
    }
}

pub enum UIEvent {
    Input(Key),
    Tick,
}

pub struct UIEvents(mpsc::Receiver<UIEvent>);

impl UIEvents {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        Self::spawn_key_handler(tx.clone());
        Self::spawn_tick_handler(tick_rate, tx);

        Self(rx)
    }

    fn spawn_key_handler(tx: mpsc::Sender<UIEvent>) -> thread::JoinHandle<()> {
        let stdin = io::stdin();

        thread::spawn(move || {
            for event in stdin.keys() {
                if let Ok(key) = event {
                    tx.send(UIEvent::Input(key)).ok();
                }
            }
        })
    }

    fn spawn_tick_handler(
        tick_rate: Duration,
        tx: mpsc::Sender<UIEvent>,
    ) -> thread::JoinHandle<()> {
        let tick_rate = tick_rate
            .to_std()
            .unwrap_or_else(|_| std::time::Duration::from_secs(1));

        thread::spawn(move || loop {
            thread::sleep(tick_rate);
            tx.send(UIEvent::Tick).ok();
        })
    }

    #[inline(always)]
    pub fn next(&self) -> Result<UIEvent> {
        self.0
            .recv()
            .context("failed to receive MPSC channel event")
    }
}
