use super::{DatabaseKeeper, DatabaseReader};

#[derive(Debug)]
pub struct SqliteDb;

impl DatabaseReader for SqliteDb {
    type Error = anyhow::Error;

    fn get_rss_title(&self, xmlurl: &str) -> Result<String, Self::Error> {
        todo!()
    }

    fn get_description(&self, xmlurl: &str) -> Result<String, Self::Error> {
        todo!()
    }

    fn get_html_url(&self, xmlurl: &str) -> Result<String, Self::Error> {
        todo!()
    }

    fn get_articles_titles(&self, xmlurl: &str) -> Result<super::titles::Titles, Self::Error> {
        todo!()
    }

    fn get_article(
        &self,
        xmlurl: &str,
        title: &str,
    ) -> Result<Option<&crate::element::Article>, Self::Error> {
        todo!()
    }
}

impl DatabaseKeeper for SqliteDb {
    type Error = anyhow::Error;

    fn save_rss_opml(
        &mut self,
        rss_channel: crate::element::RssChannel,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn save_articles(
        &mut self,
        rss_channel: crate::element::RssChannel,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
