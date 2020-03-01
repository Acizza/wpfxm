use super::{Backend, Component, Frame, Key, Rect};
use crate::tui::backend::UIBackend;
use crate::tui::{LogResult, State};
use generic_array::{ArrayLength, GenericArray};
use smallvec::SmallVec;
use std::marker::PhantomData;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};

pub struct TabList<B, N>
where
    B: Backend,
    N: ArrayLength<GenericTab<B>>,
{
    block_title: &'static str,
    items: GenericArray<GenericTab<B>, N>,
    tab_titles: SmallVec<[&'static str; 2]>,
    selected: usize,
    pub content_draw_rect: Rect,
}

impl<B, N> TabList<B, N>
where
    B: Backend,
    N: ArrayLength<GenericTab<B>>,
{
    pub fn new(block_title: &'static str, items: GenericArray<GenericTab<B>, N>) -> Self {
        let tab_titles = items.iter().map(|tab| tab.name).collect();

        Self {
            block_title,
            items,
            tab_titles,
            selected: 0,
            content_draw_rect: Rect::default(),
        }
    }
}

impl<B, N> Component<B> for TabList<B, N>
where
    B: Backend,
    N: ArrayLength<GenericTab<B>>,
{
    fn process_key(&mut self, key: Key, _: &mut State) -> LogResult {
        match key {
            Key::Char('\t') => {
                let new_index = self.selected + 1;

                self.selected = if new_index < self.items.len() {
                    new_index
                } else {
                    0
                };

                LogResult::Ok
            }
            _ => LogResult::Ok,
        }
    }

    fn draw(&mut self, state: &State, rect: Rect, frame: &mut Frame<B>) {
        Tabs::default()
            .block(
                Block::default()
                    .title(self.block_title)
                    .borders(Borders::ALL),
            )
            .titles(&self.tab_titles)
            .highlight_style(Style::default().fg(Color::Green))
            .select(self.selected)
            .render(frame, rect);

        let tab = match self.items.get_mut(self.selected) {
            Some(tab) => tab,
            None => return,
        };

        tab.content.draw(state, self.content_draw_rect, frame);
    }
}

pub struct Tab<B, C>
where
    B: Backend,
    C: Component<B> + ?Sized,
{
    pub name: &'static str,
    pub content: Box<C>,
    phantom: PhantomData<B>,
}

pub type GenericTab<B> = Tab<B, dyn Component<B>>;

impl<B, C> Tab<B, C>
where
    B: Backend,
    C: Component<B> + ?Sized,
{
    pub fn new<T>(name: &'static str, content: T) -> Self
    where
        T: Into<Box<C>>,
    {
        Self {
            name,
            content: content.into(),
            phantom: PhantomData::default(),
        }
    }
}

impl<B, C> Component<B> for Tab<B, C>
where
    B: Backend,
    C: Component<B> + ?Sized,
{
    #[inline(always)]
    fn tick(&mut self, state: &mut State) -> LogResult {
        self.content.tick(state)
    }

    #[inline(always)]
    fn process_key(&mut self, key: Key, state: &mut State) -> LogResult {
        self.content.process_key(key, state)
    }

    #[inline(always)]
    fn draw(&mut self, state: &State, rect: Rect, frame: &mut Frame<B>) {
        self.content.draw(state, rect, frame)
    }

    #[inline(always)]
    fn after_draw(&mut self, backend: &mut UIBackend<B>) {
        self.content.after_draw(backend)
    }
}
