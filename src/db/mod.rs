use crate::element::{Article, RssChannel};
use async_trait::async_trait;

use self::titles::Titles;

pub mod hashmap_db;
pub mod nosql_database;
pub mod preprocess;
pub mod sql_database;
pub mod sqlite_db;
pub mod titles;
pub mod utils;


#[async_trait]
pub trait DatabaseKeeper {
    type Error;

    async fn save(&mut self, rss_channel: RssChannel) -> Result<(), Self::Error> {
        self.save_rss_opml(rss_channel.clone()).await?;
        self.save_articles(rss_channel).await
    }

    async fn save_rss_opml(&mut self, rss_channel: RssChannel) -> Result<(), Self::Error>;

    async fn save_articles(&mut self, rss_channel: RssChannel) -> Result<(), Self::Error>;
}

#[async_trait]
// CREATE TABLE rssopml (title TEXT, description TEXT, htmlurl TEXT, xmlurl TEXT, titles TEXT);
pub trait DatabaseReader {
    type Error;

    /// get titles by xmlurl
    async fn get_rss_title(&self, xmlurl: &str) -> Result<String, Self::Error>;

    async fn get_description(&self, xmlurl: &str) -> Result<String, Self::Error>;

    async fn get_html_url(&self, xmlurl: &str) -> Result<String, Self::Error>;

    async fn get_articles_titles(&self, xmlurl: &str) -> Result<Titles, Self::Error>;

    async fn get_article(&self, xmlurl: &str, title: &str) -> Result<Option<Article>, Self::Error>;
}
