use std::{future::Future, sync::Arc};
use tokio::runtime::Runtime;

use super::{logic, modes::Selected, stateful_list::StatefulList};
use crate::{config::TitleAndRssUrl, element::Article, Config};

pub struct App {
    // config
    pub config: Config,

    // entry
    pub entry_selection_position: usize,
    pub entry_scroll_position: u16,
    pub entrys_len: usize,
    pub entry_lines_len: usize,
    pub entry_lines_rendered_len: u16,
    pub entry_column_width: u16,
    // current entry article or text
    pub current_entry_article: Option<Article>,
    pub current_entry_text: String,
    // current entry rss url
    pub currtn_entry_rss_url: String,
    // current entry title
    pub current_entry_titles: StatefulList<String>,
    // current category title feeds xml url and title
    pub current_category_items: StatefulList<TitleAndRssUrl>,

    // modes
    pub selected: Selected,
    // category
    // category title
    pub category_titles: StatefulList<String>,
    // current category title index
    pub current_category_index: usize,

    // runtime
    pub runtime: Arc<Runtime>,
}

impl App {
    pub fn new(config: Config, category_titles: Vec<String>) -> anyhow::Result<App> {
        // get current category title
        let entry_titles = config
            .outlines(0)
            .into_iter()
            .map(|value| value)
            .collect::<Vec<TitleAndRssUrl>>();

        let current_category_items = StatefulList::with_items(entry_titles.clone());
        let category_titles = StatefulList::with_items(category_titles);

        let currtn_entry_rss_url = entry_titles[0].clone().rss_url;
        let runtime = Arc::new(Runtime::new()?);

        let current_category_titles = runtime
            .block_on(logic::get_titles(&currtn_entry_rss_url))?
            .titles
            .clone();

        let current_entry_titles = StatefulList::with_items(current_category_titles);

        Ok(App {
            config,
            selected: Selected::Category,
            category_titles,
            current_category_index: 0,
            current_category_items,
            runtime,
            entry_selection_position: 0,
            current_entry_text: String::new(),
            entry_scroll_position: 0,
            entry_lines_len: 0,
            entrys_len: 0,
            entry_lines_rendered_len: 0,
            entry_column_width: 0,
            current_entry_article: None,
            current_entry_titles,
            currtn_entry_rss_url,
        })
    }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }

    // reset current entry titles
    pub fn reset_current_entry_titles(&mut self) -> anyhow::Result<()> {
        let current_category_titles = self
            .block_on(logic::get_titles(&self.currtn_entry_rss_url))?
            .titles
            .clone();

        self.current_entry_titles = StatefulList::with_items(current_category_titles);
        Ok(())
    }

    pub fn reset_current_entry_items(&mut self) {
        let entry_titles = self
            .config
            .outlines(self.current_category_index)
            .into_iter()
            .map(|value| value)
            .collect::<Vec<TitleAndRssUrl>>();

        let current_category_items = StatefulList::with_items(entry_titles.clone());

        self.current_category_items = current_category_items;
    }

    fn update_entry_selection_position(&mut self) {
        if self.current_category_items.items.is_empty() {
            self.entry_selection_position = 0
        } else if self.entry_selection_position > self.current_category_items.items.len() - 1 {
            self.entry_selection_position = self.current_category_items.items.len() - 1
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
            Selected::Feeds => {
                self.selected = Selected::Category;
            }
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
            Selected::Category => {}
        }

        Ok(())
    }

    pub fn on_right(&mut self) -> anyhow::Result<()> {
        match self.selected {
            Selected::Feeds => {
                // if !self.current_category_items.items.is_empty() {
                //     self.selected = Selected::Entries;
                //     self.reset_current_entry_titles()?;
                // }
                Ok(())
            }
            Selected::Entries => self.on_enter(),
            Selected::Entry => Ok(()),
            Selected::None => Ok(()),
            Selected::Category => {
                // self.entry_selection_position = 0;

                // self.currtn_entry_rss_url = self.current_category_items.items[self.entry_selection_position].clone().rss_url;

                self.selected = Selected::Feeds;
                self.reset_current_entry_items();

                Ok(())
            }
        }
    }

    pub fn on_up(&mut self) -> anyhow::Result<()> {
        match self.selected {
            Selected::Feeds => {
                self.current_category_items.previous();

                // update the currtn_entry_rss_url
                if self.entry_selection_position > 0 {
                    self.entry_selection_position -= 1;
                } else {
                    self.entry_selection_position = self.current_category_items.items.len() - 1;
                }

                self.currtn_entry_rss_url = self.current_category_items.items
                    [self.entry_selection_position]
                    .clone()
                    .rss_url;

                // update current_entry_titles
                // self.reset_current_entry_titles()?;
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
            Selected::Category => {
                self.category_titles.previous();

                if self.current_category_index > 0 {
                    self.current_category_index -= 1;
                } else {
                    self.current_category_index = self.category_titles.items.len() - 1;
                }

                let titles = self.current_category_rss_url(self.current_category_index);
                self.entrys_len = titles.len();

                self.current_category_items = StatefulList::with_items(titles);
            }
        }

        Ok(())
    }

    pub fn on_down(&mut self) -> anyhow::Result<()> {
        match self.selected {
            Selected::Feeds => {
                self.current_category_items.next();

                // update the currtn_entry_rss_url
                self.entry_selection_position =
                    (self.entry_selection_position + 1) % self.current_category_items.items.len();

                self.currtn_entry_rss_url = self.current_category_items.items
                    [self.entry_selection_position]
                    .clone()
                    .rss_url;

                // update current_entry_titles
                // self.reset_current_entry_titles()?;
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
            Selected::Category => {
                self.category_titles.next();
                self.current_category_index =
                    (self.current_category_index + 1) % self.category_titles.items.len();

                // update current Category feeds xml url
                let titles = self.current_category_rss_url(self.current_category_index);

                self.current_category_items = StatefulList::with_items(titles);
            }
        }

        Ok(())
    }

    /// get the current category rss urls
    pub fn current_category_rss_url(&self, index: usize) -> Vec<TitleAndRssUrl> {
        self.config
            .outlines(index)
            .into_iter()
            .map(|value| value)
            .collect::<Vec<TitleAndRssUrl>>()
    }
}
