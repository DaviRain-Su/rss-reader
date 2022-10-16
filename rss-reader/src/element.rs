use std::fmt::{Display, self};
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssChannel {
    pub channel_title: String,
    pub channel_link: String,
    pub description: String,
    pub image: Option<RssImage>,
    pub items: Vec<RssItem>,
}

impl RssChannel {
    pub fn rss_url(&self) -> &str {
        &self.channel_link
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssImage {
    pub image_name: String,
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssItem {
    pub title: Option<String>,
    pub link: Option<String>,
    pub public_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Articles {
    pub channel_title: String,
    pub channel_link: String,
    pub description: String,
    pub image: Option<RssImage>,
    pub articles: Vec<Article>,
}

impl Default for Articles {
    fn default() -> Self {
        Self {
            channel_title: Default::default(),
            channel_link: Default::default(),
            description: Default::default(),
            image: None,
            articles: vec![],
        }
    }
}

impl Display for Articles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.channel_title.green())?;
        write!(f, "{}", self.channel_link.green())?;
        write!(f, "{}", self.description.black())?;
        for article in self.articles.iter() {
            write!(f, "{}", article)?;
            write!(f, "\n\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,        // Element(<title>)
    pub phases: Vec<Section>, // Element(<p>)
}

impl Display for Article { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "🌞Title: {}🌞\n", self.title.green())?;
        // TODO(davirain) get screen width
        write!(f, "------------------------------------\n")?;
        for (idx, section) in self.phases.iter().enumerate() {
            if idx % 2 == 0 {
                write!(f, "    {}\n", section.to_string().bright_red())?;
            } else {
                write!(f, "    {}\n", section.to_string().yellow())?;
            }
            
        }
        Ok(())
    }
}

impl Default for Article {
    fn default() -> Self {
        Self {
            title: Default::default(),
            phases: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    content: String,
}

impl Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl RssChannel {
    pub async fn process_rss_channel_to_article(&self) -> anyhow::Result<Articles> {
        let mut articles = Articles::default();
        articles.channel_title = self.channel_title.clone();
        articles.channel_link = self.channel_link.clone();
        articles.description = self.description.clone();
        articles.image = self.image.clone();

        let mut item_articles = vec![];

        for item in self.items.iter() {
            let mut article = Article::default();

            let link = item.link.clone().unwrap_or_default();
            let response = reqwest::get(link).await?.text().await?;
            let dom = tl::parse(&response, tl::ParserOptions::new().track_ids()).unwrap();

            // parse mirror title
            let handle = dom
                .query_selector("title")
                .and_then(|mut iter| iter.next())
                .unwrap();
            let node = handle.get(dom.parser()).unwrap();

            let article_title = node.inner_text(dom.parser());
            article.title = article_title.into();

            let _ = dom.query_selector("p").map(|mut iter| loop {
                if let Some(handle) = iter.next() {
                    let node = handle.get(dom.parser()).unwrap();
                    let section = node.inner_text(dom.parser()).to_string();
                    article.phases.push(Section { content: section });
                } else {
                    break;
                }
            });

            item_articles.push(article);
        }

        articles.articles = item_articles;

        Ok(articles)
    }
}
