use std::collections::HashMap;

use crate::element::{RssChannel, Article};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static GLOBAL_DATA: Lazy<Mutex<Db>> = Lazy::new(|| Mutex::new(Db::default()));

#[derive(Debug)]
pub struct Db {
    /// key is subscribe address
    /// value is subscribe mirror url
    pub sub_mirror: HashMap<String, String>,
    /// key is subscribe address
    /// value is subscribe mirror Content
    pub rss_channels: HashMap<String, Vec<String>>,
    /// key article author
    /// value they are articles
    pub articles: HashMap<String, Article>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            sub_mirror: Default::default(),
            rss_channels: Default::default(),
            articles: Default::default(),
        }
    }
}

impl Db {
    pub fn save(&mut self, address: String, rss_channel: RssChannel) {
        self.sub_mirror
            .insert(address.clone(), rss_channel.rss_url().to_owned());
        // self.rss_channels.insert(address, rss_channel);
        todo!()
    }

    pub fn get(&self, address: String) -> Option<&RssChannel> {
        // self.rss_channels.get(&address)
        todo!()
    }
}
