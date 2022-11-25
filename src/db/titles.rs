use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Titles {
    titles: Vec<String>,
}

impl Default for Titles {
    fn default() -> Self {
        Self {
            titles: vec!["1".into(), "2".into(), "3".into()],
        }
    }
}
