use once_cell::sync::Lazy;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::element::{Articles, RssChannel};

pub static GLOBAL_DATA: Lazy<Mutex<Db>> = Lazy::new(|| Mutex::new(Db::default()));

#[derive(Debug)]
pub struct Db {
    /// key is subscribe address
    /// value is mirror url
    pub rss_people_ursl: HashMap<String, BTreeSet<String>>,
    /// key is subscribe address
    /// value is subscribe mirror author address
    pub rss_peoples: HashMap<String, BTreeSet<String>>,
    /// key article author address
    /// value they are articles
    pub articles: HashMap<String, Articles>,
    /// key: article author address
    /// value: they are rss channel
    pub rss_channels: HashMap<String, RssChannel>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            rss_people_ursl: Default::default(),
            rss_peoples: Default::default(),
            articles: Default::default(),
            rss_channels: Default::default(),
        }
    }
}

impl Db {
    pub async fn save(&mut self, address: String, rss_channel: RssChannel) -> anyhow::Result<()> {
        let mirror_url = rss_channel.channel_link.clone();
        let (_, mirror_address) = mirror_url.rsplit_once("/").unwrap_or_default();
        let mut new_mirror_address = String::new();
        if mirror_address.contains(".mirror.xyz") {
            new_mirror_address = mirror_url
                .split_once(".mirror.xyz")
                .unwrap_or_default()
                .0
                .split_once("//")
                .unwrap_or_default()
                .1
                .to_string();
            new_mirror_address.push_str(".eth");
        } else {
            new_mirror_address = mirror_address.to_string();
        }
        let mirror_address = new_mirror_address;
        // todo(log)
        // println!("{:?}", mirror_address);

        if self.rss_peoples.contains_key(&address) {
            if let Some(value) = self.rss_people_ursl.get_mut(&address) {
                value.insert(mirror_url.clone());
            }

            if let Some(value) = self.rss_peoples.get_mut(&address) {
                value.insert(mirror_address.clone());
            }
        } else {
            let mut btee_set_url = BTreeSet::new();
            btee_set_url.insert(mirror_url.clone());
            self.rss_people_ursl.insert(address.clone(), btee_set_url);

            let mut btee_set_address = BTreeSet::new();
            btee_set_address.insert(mirror_address.clone());
            self.rss_peoples.insert(address, btee_set_address);
        }

        // save rss_channels
        // key: articles address
        self.rss_channels
            .insert(mirror_address.clone(), rss_channel.clone());

        // save artivles
        let temp_articles = rss_channel.process_rss_channel_to_article().await?;
        self.articles.insert(mirror_address.clone(), temp_articles);

        Ok(())
    }

    pub fn get_rss_channel(
        &self,
        user_address: String,
        subscribe_author: String,
    ) -> anyhow::Result<&RssChannel> {
        if let Some(subscribe_authors) = self.rss_peoples.get(&user_address) {
            if !subscribe_authors.contains(&subscribe_author) {
                return Err(anyhow::anyhow!(format!(
                    "This Uset have not subcribe: {}",
                    subscribe_author
                )));
            }
        } else {
            return Err(anyhow::anyhow!(
                "This User have not subscibe any mirror author!"
            ));
        }

        self.rss_channels
            .get(&subscribe_author)
            .ok_or(anyhow::anyhow!("This author have not any articles"))
    }

    pub fn get_rss_articles(
        &self,
        user_address: String,
        subscribe_author: String,
    ) -> anyhow::Result<&Articles> {
        if let Some(subscribe_authors) = self.rss_peoples.get(&user_address) {
            if !subscribe_authors.contains(&subscribe_author) {
                return Err(anyhow::anyhow!(format!(
                    "This Uset have not subcribe: {}",
                    subscribe_author
                )));
            }
        } else {
            return Err(anyhow::anyhow!(
                "This User have not subscibe any mirror author!"
            ));
        }

        self.articles
            .get(&subscribe_author)
            .ok_or(anyhow::anyhow!("This author have not any articles"))
    }
}
