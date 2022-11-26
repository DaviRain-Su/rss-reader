use std::{future::Future, sync::Arc};
use tokio::runtime::Runtime;

use super::{modes::Selected, stateful_list::StatefulList};
use crate::{config::TitleAndRssUrl, element::Article, Config};

pub struct App<'a> {
    // config
    pub config: Config,

    // entry
    pub entry_selection_position: usize,
    pub current_entry_article: Option<Article>,
    pub current_entry_text: String,
    pub entry_scroll_position: u16,
    pub entry_lines_len: usize,
    pub entry_lines_rendered_len: u16,
    pub entry_column_width: u16,

    // modes
    pub selected: Selected,
    // tabs
    pub tabs_titles: Vec<&'a str>,
    pub tabs_index: usize,
    pub current_tab_items: Vec<StatefulList<(TitleAndRssUrl, usize)>>,

    // runtime
    pub runtime: Arc<Runtime>,
}

impl<'a> App<'a> {
    pub fn new(config: Config, tabs_titles: Vec<&'a str>) -> anyhow::Result<App<'a>> {
        let mut current_tab_items = vec![];

        let titles = config
            .outlines(0)
            .into_iter()
            .enumerate()
            .map(|(idx, value)| (value, idx))
            .collect::<Vec<(TitleAndRssUrl, usize)>>();

        current_tab_items.push(StatefulList::with_items(titles));

        let runtime = Arc::new(Runtime::new()?);

        Ok(App {
            config,
            selected: Selected::None,
            tabs_titles,
            tabs_index: 0,
            current_tab_items,
            runtime,
            entry_selection_position: 0,
            current_entry_text: String::new(),
            entry_scroll_position: 0,
            entry_lines_len: 0,
            entry_lines_rendered_len: 0,
            entry_column_width: 0,
            current_entry_article: None,
        })
    }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }

    pub fn next(&mut self) {
        self.tabs_index = (self.tabs_index + 1) % self.tabs_titles.len();

        if let Some(_value) = self.current_tab_items.get(self.tabs_index) {
        } else {
            let titles = self
                .config
                .outlines(self.tabs_index)
                .into_iter()
                .enumerate()
                .map(|(idx, value)| (value, idx))
                .collect::<Vec<(TitleAndRssUrl, usize)>>();

            self.current_tab_items
                .push(StatefulList::with_items(titles));
        }
    }

    pub fn previous(&mut self) {
        if self.tabs_index > 0 {
            self.tabs_index -= 1;
        } else {
            self.tabs_index = self.tabs_titles.len() - 1;
        }

        if let Some(_value) = self.current_tab_items.get(self.tabs_index) {
        } else {
            let titles = self
                .config
                .outlines(self.tabs_index)
                .into_iter()
                .enumerate()
                .map(|(idx, value)| (value, idx))
                .collect::<Vec<(TitleAndRssUrl, usize)>>();

            self.current_tab_items
                .push(StatefulList::with_items(titles));
        }
    }

    fn update_entry_selection_position(&mut self) {
        if self.current_tab_items.is_empty() {
            self.entry_selection_position = 0
        } else if self.entry_selection_position > self.current_tab_items.len() - 1 {
            self.entry_selection_position = self.current_tab_items.len() - 1
        };
    }

    // entry page up
    pub fn page_up(&mut self) {
        if matches!(self.selected, Selected::Entry) {
            self.entry_scroll_position = if let Some(position) = self
                .entry_scroll_position
                .checked_sub(self.entry_lines_rendered_len)
            {
                position
            } else {
                0
            }
        }
    }

    // entry page down
    pub fn page_down(&mut self) {
        if matches!(self.selected, Selected::Entry) {
            self.entry_scroll_position = if self.entry_scroll_position
                + self.entry_lines_rendered_len
                >= self.entry_lines_len as u16
            {
                self.entry_lines_len as u16
            } else {
                self.entry_scroll_position + self.entry_lines_rendered_len
            }
        }
    }

    // when enter
    // todo
    pub fn on_enter(&mut self) -> anyhow::Result<()> {
        match self.selected {
            Selected::Entries | Selected::Entry => {
                let empty_string = String::from("No content or description tag provided.");

                // minimum is 1
                let line_length = if self.entry_column_width >= 5 {
                    self.entry_column_width - 4
                } else {
                    1
                };

                self.selected = Selected::Entry;

                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn on_left(&mut self) -> anyhow::Result<()> {
        match self.selected {
            Selected::Feeds => (),
            Selected::Entries => {
                self.entry_selection_position = 0;
                self.selected = Selected::Feeds
            }
            Selected::Entry => {
                self.entry_scroll_position = 0;
                self.selected = {
                    self.current_entry_text = String::new();
                    Selected::Entries
                }
            }
            Selected::None => (),
            Selected::Tabs => todo!(),
        }

        Ok(())
    }

    pub fn on_up(&mut self) -> anyhow::Result<()> {
        match self.selected {
            Selected::Feeds => {
                // self.feeds.previous();
                // self.update_current_feed_and_entries()?;
            }
            Selected::Entries => {
                // if !self.items.is_empty() {
                    // self.entries.previous();
                    // self.entry_selection_position = self.entries.state.selected().unwrap();
                    // self.update_current_entry_meta()?;
                // }
            }
            Selected::Entry => {
                if let Some(n) = self.entry_scroll_position.checked_sub(1) {
                    self.entry_scroll_position = n
                };
            }
            Selected::None => (),
            Selected::Tabs => todo!(),
        }

        Ok(())
    }

    pub fn on_right(&mut self) -> anyhow::Result<()> {
        match self.selected {
            Selected::Feeds => {
                // if !self.entries.items.is_empty() {
                    // self.selected = Selected::Entries;
                    // self.entries.reset();
                    // self.update_current_entry_meta()?;
                // }
                Ok(())
            }
            Selected::Entries => self.on_enter(),
            Selected::Entry => Ok(()),
            Selected::None => Ok(()),
            Selected::Tabs => todo!(),
        }
    }

    pub fn on_down(&mut self) -> anyhow::Result<()> {
        match self.selected {
            Selected::Feeds => {
                // self.feeds.next();
                // self.update_current_feed_and_entries()?;
            }
            Selected::Entries => {
                // if !self.entries.items.is_empty() {
                    // self.entries.next();
                    // self.entry_selection_position = self.entries.state.selected().unwrap();
                    // self.update_current_entry_meta()?;
                // }
            }
            Selected::Entry => {
                if let Some(n) = self.entry_scroll_position.checked_add(1) {
                    self.entry_scroll_position = n
                };
            }
            Selected::None => (),
            Selected::Tabs => todo!(),
        }

        Ok(())
    }
}
