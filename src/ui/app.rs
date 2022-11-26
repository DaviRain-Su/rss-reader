use std::{sync::Arc, future::Future};
use tokio::runtime::Runtime;

use super::stateful_list::StatefulList;
use crate::{config::TitleAndRssUrl, Config};

pub struct App<'a> {
    pub config: Config,
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub items: Vec<StatefulList<(TitleAndRssUrl, usize)>>,
    pub runtime: Arc<Runtime>,
}

impl<'a> App<'a> {
    pub fn new(config: Config, categoryes: Vec<&'a str>) -> anyhow::Result<App<'a>> {
        let mut items = vec![];

        let titles = config
            .outlines(0)
            .into_iter()
            .enumerate()
            .map(|(idx, value)| (value, idx))
            .collect::<Vec<(TitleAndRssUrl, usize)>>();

        items.push(StatefulList::with_items(titles));

        let runtime = Arc::new(Runtime::new()?);

        Ok(App {
            config,
            titles: categoryes,
            index: 0,
            items,
            runtime,
        })
    }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();

        if let Some(_value) = self.items.get(self.index) {
        } else {
            let titles = self
                .config
                .outlines(self.index)
                .into_iter()
                .enumerate()
                .map(|(idx, value)| (value, idx))
                .collect::<Vec<(TitleAndRssUrl, usize)>>();

            self.items.push(StatefulList::with_items(titles));
        }
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }

        if let Some(_value) = self.items.get(self.index) {
        } else {
            let titles = self
                .config
                .outlines(self.index)
                .into_iter()
                .enumerate()
                .map(|(idx, value)| (value, idx))
                .collect::<Vec<(TitleAndRssUrl, usize)>>();

            self.items.push(StatefulList::with_items(titles));
        }
    }
}
