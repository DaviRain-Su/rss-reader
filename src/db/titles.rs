use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Titles {
    pub titles: Vec<String>,
}

impl Default for Titles {
    fn default() -> Self {
        Self {
            titles: vec!["default title".into()],
        }
    }
}
