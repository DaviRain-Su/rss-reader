use opml::{Body, OPML};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    rss_source_file: OPML,
}

impl Config {
    pub fn from_str(file: &str) -> Result<Self, anyhow::Error> {
        let document = OPML::from_str(file)?;
        Ok(Self {
            rss_source_file: document,
        })
    }

    pub fn rss_source_file(&self) -> &OPML {
        &self.rss_source_file
    }

    pub fn body(&self) -> &Body {
        &self.rss_source_file.body
    }

    pub fn category_len(&self) -> usize {
        self.rss_source_file.body.outlines.len()
    }

    pub fn category(&self) -> Vec<String> {
        let result = self
            .body()
            .outlines
            .iter()
            .map(|value| format!("{}", value.title.clone().unwrap_or("default".to_string())))
            .collect::<Vec<_>>();

        result
    }
}
