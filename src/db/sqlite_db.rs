use super::{
    nosql_database,
    sql_database::{self, get_article_titles, get_description, get_htmlurl, get_rss_title},
    utils, DatabaseKeeper, DatabaseReader,
};

#[derive(Debug)]
pub struct SqliteDb;

impl DatabaseReader for SqliteDb {
    type Error = anyhow::Error;

    fn get_rss_title(&self, xmlurl: &str) -> Result<String, Self::Error> {
        get_rss_title(xmlurl)
    }

    fn get_description(&self, xmlurl: &str) -> Result<String, Self::Error> {
        get_description(xmlurl)
    }

    fn get_html_url(&self, xmlurl: &str) -> Result<String, Self::Error> {
        get_htmlurl(xmlurl)
    }

    fn get_articles_titles(&self, xmlurl: &str) -> Result<super::titles::Titles, Self::Error> {
        get_article_titles(xmlurl)
    }

    fn get_article(
        &self,
        xmlurl: &str,
        title: &str,
    ) -> Result<Option<crate::element::Article>, Self::Error> {
        let key = utils::hash(format!("{}{}", xmlurl, title).as_bytes());
        let result = nosql_database::get(&key)?;

        let result: Option<crate::element::Article> =
            serde_json::from_slice(&result).unwrap_or(None);

        Ok(result)
    }
}

impl DatabaseKeeper for SqliteDb {
    type Error = anyhow::Error;

    fn save_rss_opml(
        &mut self,
        rss_channel: crate::element::RssChannel,
    ) -> Result<(), Self::Error> {
        match sql_database::create_rss_database() {
            Ok(_) => println!("Create database successful!"),
            Err(e) => println!("{:?}", e),
        }

        let title = rss_channel.channel_title.clone();
        let description = rss_channel.description.clone();
        let htmlurl = rss_channel.channel_html_url.clone();
        let xmlurl = rss_channel.channel_xml_url.clone();
        let titles = rss_channel.process_rss_channel_to_article()?.titles();

        sql_database::insert(title, description, htmlurl, xmlurl, titles)
    }

    fn save_articles(
        &mut self,
        rss_channel: crate::element::RssChannel,
    ) -> Result<(), Self::Error> {
        let xmlurl = rss_channel.channel_xml_url.clone();
        let articles = rss_channel.process_rss_channel_to_article()?.articles();

        for item in articles.iter() {
            let title = item.title.clone();
            let key = utils::hash(format!("{}{}", xmlurl, title).as_bytes());
            let value = serde_json::to_string(item)?;
            nosql_database::insert(&key, value.clone().as_bytes())?;
        }

        Ok(())
    }
}
