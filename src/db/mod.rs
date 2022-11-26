use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::element::{Articles, RssChannel};

use self::titles::Titles;

pub mod nosql_database;
pub mod preprocess;
pub mod sql_database;
pub mod titles;
pub mod utils;

pub trait DatabaseKeeper {
    type Error;

    fn save(&mut self, rss_channel: RssChannel) -> Result<(), Self::Error> {
        self.save_rss_opml(rss_channel.clone())?;
        self.save_articles(rss_channel)
    }

    fn save_rss_opml(&mut self, rss_channel: RssChannel) -> Result<(), Self::Error>;

    fn save_articles(&mut self, rss_channel: RssChannel) -> Result<(), Self::Error>;
}

// CREATE TABLE rssopml (title TEXT, description TEXT, htmlurl TEXT, xmlurl TEXT, titles TEXT);
pub trait DatabaseReader {
    type Error;

    /// get titles by xmlurl
    fn get_rss_title(&self, xmlurl: &str) -> Result<String, Self::Error>;

    fn get_description(&self, xmlurl: &str) -> Result<String, Self::Error>;

    fn get_html_url(&self, xmlurl: &str) -> Result<String, Self::Error>;

    fn get_articles_titles(&self, xmlurl: &str) -> Result<Titles, Self::Error>;
}

pub static GLOBAL_DATA: Lazy<Mutex<Db>> = Lazy::new(|| Mutex::new(Db::default()));
#[derive(Debug)]
pub struct Db {
    /// key xml url
    /// value they are articles
    pub articles: HashMap<String, Articles>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            articles: Default::default(),
        }
    }
}

impl Db {
    pub fn save(&mut self, rss_channel: RssChannel) -> anyhow::Result<()> {
        let xml_url = rss_channel.channel_xml_url.clone();

        // save artivles
        let temp_articles = rss_channel.process_rss_channel_to_article()?;
        self.articles
            .insert(xml_url.to_string().clone(), temp_articles);

        Ok(())
    }

    pub fn rss_articles(&self, xmlurl: &str) -> anyhow::Result<&Articles> {
        self.articles
            .get(xmlurl)
            .ok_or(anyhow::anyhow!("This {} have not any articles", xmlurl))
    }

    pub fn rss_titles(&self, xmlurl: &str) -> anyhow::Result<Titles> {
        let titles = self
            .articles
            .get(xmlurl)
            .ok_or(anyhow::anyhow!("This {} have not any articles", xmlurl))?
            .articles
            .iter()
            .map(|value| value.title.clone())
            .collect();

        Ok(Titles { titles })
    }

    pub fn rss_title(&self, xmlurl: &str) -> anyhow::Result<String> {
        let title = self
            .articles
            .get(xmlurl)
            .ok_or(anyhow::anyhow!("This {} have not any articles", xmlurl))?
            .channel_title
            .clone();

        Ok(title)
    }

    fn description(&self, xmlurl: &str) -> anyhow::Result<String> {
        let description = self
            .articles
            .get(xmlurl)
            .ok_or(anyhow::anyhow!("This {} have not any articles", xmlurl))?
            .description
            .clone();

        Ok(description)
    }

    fn html_url(&self, xmlurl: &str) -> anyhow::Result<String> {
        let result = self
            .articles
            .get(xmlurl)
            .ok_or(anyhow::anyhow!("This {} have not any articles", xmlurl))?
            .channel_html_url
            .clone();

        Ok(result)
    }
}

impl DatabaseKeeper for Db {
    type Error = anyhow::Error;

    fn save_rss_opml(&mut self, _rss_channel: RssChannel) -> Result<(), Self::Error> {
        Ok(())
    }

    fn save_articles(&mut self, rss_channel: RssChannel) -> Result<(), Self::Error> {
        self.save(rss_channel)
    }
}

impl DatabaseReader for Db {
    type Error = anyhow::Error;

    fn get_rss_title(&self, xmlurl: &str) -> Result<String, Self::Error> {
        self.rss_title(xmlurl)
    }

    fn get_description(&self, xmlurl: &str) -> Result<String, Self::Error> {
        self.description(xmlurl)
    }

    fn get_html_url(&self, xmlurl: &str) -> Result<String, Self::Error> {
        self.html_url(xmlurl)
    }

    fn get_articles_titles(&self, xmlurl: &str) -> Result<Titles, Self::Error> {
        self.rss_titles(xmlurl)
    }
}
