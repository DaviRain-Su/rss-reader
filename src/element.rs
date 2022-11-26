use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

use crate::db::titles::Titles;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssChannel {
    pub channel_title: String,
    pub channel_html_url: String,
    pub channel_xml_url: String,
    pub description: String,
    pub image: Option<RssImage>,
    pub items: Vec<RssItem>,
}

impl RssChannel {
    pub fn channel_html_url(&self) -> &str {
        &self.channel_html_url
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
    pub channel_html_url: String,
    pub channel_xml_url: String,
    pub description: String,
    pub image: Option<RssImage>,
    pub articles: Vec<Article>,
}

impl Articles {
    pub fn with_channel_title(mut self, channel_title: String) -> Self {
        self.channel_title = channel_title;
        self
    }

    pub fn with_channel_html_url(mut self, channel_html_url: String) -> Self {
        self.channel_html_url = channel_html_url;
        self
    }

    pub fn with_chanenl_xml_url(mut self, channel_xml_url: String) -> Self {
        self.channel_xml_url = channel_xml_url;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_image(mut self, image: Option<RssImage>) -> Self {
        self.image = image;
        self
    }

    pub fn with_articles(mut self, articles: Vec<Article>) -> Self {
        self.articles = articles;
        self
    }

    pub fn articles(&self) -> Vec<Article> {
        self.articles.clone()
    }

    pub fn titles(&self) -> Titles {
        let titles = self
            .articles
            .iter()
            .map(|value| value.title.clone())
            .collect::<Vec<_>>();

        Titles { titles }
    }
}

impl Default for Articles {
    fn default() -> Self {
        Self {
            channel_title: Default::default(),
            channel_html_url: Default::default(),
            channel_xml_url: Default::default(),
            description: Default::default(),
            image: None,
            articles: vec![],
        }
    }
}

impl Display for Articles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.channel_title.green())?;
        write!(f, "{}", self.channel_html_url.green())?;
        write!(f, "{}", self.channel_xml_url.green())?;
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
        let mut articles = Articles::default()
            .with_channel_title(self.channel_title.clone())
            .with_channel_html_url(self.channel_html_url.clone())
            .with_chanenl_xml_url(self.channel_xml_url.clone())
            .with_description(self.description.clone())
            .with_image(self.image.clone());

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
                .ok_or(anyhow::anyhow!("unknown title"))?;

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
