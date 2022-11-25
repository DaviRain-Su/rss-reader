use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::{
    element::{Articles, RssChannel},
    utils::get_author_address_or_name,
};

pub mod nosql_database;
pub mod sql_database;
pub mod titles;
pub mod utils;
pub mod preprocess;

pub static GLOBAL_DATA: Lazy<Mutex<Db>> = Lazy::new(|| Mutex::new(Db::default()));

#[derive(Debug)]
pub struct Db {
    /// key article author address
    /// value they are articles
    pub articles: HashMap<String, Articles>,
    /// key: article author address
    /// value: they are rss channel
    pub rss_channels: HashMap<String, RssChannel>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            articles: Default::default(),
            rss_channels: Default::default(),
        }
    }
}

impl Db {
    pub fn save(&mut self, rss_channel: RssChannel) -> anyhow::Result<()> {
        let mirror_url = rss_channel.channel_link.clone();
        let mirror_address = get_author_address_or_name(&mirror_url);

        // save rss_channels
        // key: articles address
        self.rss_channels
            .insert(mirror_address.to_string().clone(), rss_channel.clone());

        // save artivles
        let temp_articles = rss_channel.process_rss_channel_to_article()?;
        self.articles
            .insert(mirror_address.to_string().clone(), temp_articles);

        Ok(())
    }

    pub fn get_rss_channel(&self, subscribe_author: String) -> anyhow::Result<&RssChannel> {
        self.rss_channels
            .get(&subscribe_author)
            .ok_or(anyhow::anyhow!("This author have not any articles"))
    }

    pub fn get_rss_articles(&self, subscribe_author: String) -> anyhow::Result<&Articles> {
        self.articles
            .get(&subscribe_author)
            .ok_or(anyhow::anyhow!("This author have not any articles"))
    }

    pub fn get_rss_titles(&self, subscribe_author: String) -> anyhow::Result<Vec<String>> {
        let result = self
            .articles
            .get(&subscribe_author)
            .ok_or(anyhow::anyhow!("This author have not any articles"))?
            .articles
            .iter()
            .map(|value| value.title.clone())
            .collect();

        Ok(result)
    }
}
