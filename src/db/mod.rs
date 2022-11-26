use crate::element::{RssChannel, Article};

use self::titles::Titles;

pub mod nosql_database;
pub mod preprocess;
pub mod sql_database;
pub mod titles;
pub mod utils;
pub mod hashmap_db;

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

    fn get_article(&self, xmlurl: &str, title: &str) -> Result<Option<&Article>, Self::Error>;
}
