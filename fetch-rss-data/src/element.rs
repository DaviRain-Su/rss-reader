use serde::{Serialize, Deserialize};

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
pub struct Article {
    pub title: String, // Element(<title>)
    pub phases: Vec<String>, // Element(<p>)
}