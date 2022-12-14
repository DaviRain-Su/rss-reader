use opml::{Body, OPML};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    rss_source_file: OPML,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TitleAndRssUrl {
    pub title: String,
    pub rss_url: String,
}

impl TitleAndRssUrl {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn rss_url(&self) -> &str {
        &self.rss_url
    }
}

impl Config {
    /// construct config
    pub fn from_str(file: &str) -> Result<Self, anyhow::Error> {
        let document = OPML::from_str(file)?;
        Ok(Self {
            rss_source_file: document,
        })
    }

    /// get rss source file
    pub fn rss_source_file(&self) -> &OPML {
        &self.rss_source_file
    }

    pub fn body(&self) -> &Body {
        &self.rss_source_file.body
    }

    /// category length
    pub fn category_len(&self) -> usize {
        self.rss_source_file.body.outlines.len()
    }

    /// get all category
    pub fn category(&self) -> Vec<String> {
        let result = self
            .body()
            .outlines
            .iter()
            .map(|value| format!("{}", value.title.clone().unwrap_or("default".to_string())))
            .collect::<Vec<_>>();

        result
    }

    /// get all rss url
    pub fn get_all_rss_url(&self) -> Vec<String> {
        let mut result = Vec::new();

        for item in self.body().outlines.iter() {
            for i in item.outlines.iter() {
                result.push(i.xml_url.clone().unwrap_or_default()); // todo: fix unwrap_or_default()
            }
        }

        result
    }

    pub fn outlines(&self, index: usize) -> Vec<TitleAndRssUrl> {
        let mut result = vec![];
        if let Some(outline) = self.body().outlines.get(index) {
            for (idx, item) in outline.outlines.iter().enumerate() {
                let value = TitleAndRssUrl {
                    title: format!("🎈{}: {}", idx, item.title.clone().unwrap_or_default()),
                    rss_url: format!("{}", item.xml_url.clone().unwrap_or_default()),
                };
                result.push(value);
            }
        }
        result
    }
}
