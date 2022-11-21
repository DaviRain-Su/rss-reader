use super::stateful_list::StatefulList;
use crate::Config;

pub struct App<'a> {
    pub config: Config,
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub items: Vec<StatefulList<(String, usize)>>,
}

impl<'a> App<'a> {
    pub fn new(config: Config, categoryes: Vec<&'a str>) -> App<'a> {
        let mut items = vec![];

        let titles = config
            .outlines(0)
            .into_iter()
            .enumerate()
            .map(|(idx, value)| (value.title.clone(), idx))
            .collect::<Vec<(String, usize)>>();

        items.push(StatefulList::with_items(titles));

        App {
            config,
            titles: categoryes,
            index: 0,
            items,
        }
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
                .map(|(idx, value)| (value.title.clone(), idx))
                .collect::<Vec<(String, usize)>>();

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
                .map(|(idx, value)| (value.title.clone(), idx))
                .collect::<Vec<(String, usize)>>();

            self.items.push(StatefulList::with_items(titles));
        }
    }
}
