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
    fn title(&self) -> &str {
        &self.title
    }
    
    fn rss_url(&self) -> &str {
        &self.rss_url
    }
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

    pub fn outlines(&self, index: usize) -> Vec<TitleAndRssUrl> {
        let mut result = vec![];
        if let Some(outline) = self.body().outlines.get(index) {
            for (idx, item) in outline.outlines.iter().enumerate() {
                let value = TitleAndRssUrl {
                    title: format!(
                        "🎈{}: {}",
                        idx,
                        item.title.clone().unwrap_or_default()
                    ),
                    rss_url: format!("{}", item.xml_url.clone().unwrap_or_default())
                };
                result.push(value);
            }
        }
        result
    }
}
