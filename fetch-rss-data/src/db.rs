use std::collections::HashMap;

use crate::element::RssChannel;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static GLOBAL_DATA: Lazy<Mutex<Db>> = Lazy::new(|| {
    Mutex::new(Db::default())
});

#[derive(Debug)]
pub struct Db {
    /// key is subscribe address
    /// value is subscribe mirror url
    pub sub_mirror: HashMap<String, String>,
    /// key is subscribe address
    /// value is subscribe mirror Content
    pub rss_channels: HashMap<String, RssChannel>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            sub_mirror: Default::default(),
            rss_channels: Default::default(),
        }
    }
}

impl Db {
    pub fn save(&mut self, address: String, rss_channel: RssChannel) {
        self.sub_mirror.insert(address.clone(), rss_channel.rss_url().to_owned());
        self.rss_channels.insert(address, rss_channel);
    }

    pub fn get(&self, address: String) -> Option<&RssChannel>{ 
        self.rss_channels.get(&address)
    }
}

