use once_cell::sync::Lazy;
use rss::Channel;
use std::sync::Mutex;

use crate::components::{RssChannel, RssImage, RssItem};
use crate::db::Db;

pub async fn process(user_address: String, channel: Channel, db: &Lazy<Mutex<Db>>) -> anyhow::Result<()> {
    let channel_title = channel.title.clone();
    let channel_link = channel.link.clone();
    let channel_description = channel.description.clone();

    let rss_image = if let Some(image) = channel.image.clone() {
        Some(RssImage {
            image_name: image.title.clone(),
            image_url: image.url.clone(),
        })
    } else {
        None
    };

    let items = channel
        .items
        .clone()
        .into_iter()
        .map(|item| RssItem {
            title: item.title.clone(),
            link: item.link.clone(),
            public_date: item.pub_date.clone(),
        })
        .collect::<Vec<RssItem>>();

    let rss_channel = RssChannel {
        channel_title,
        channel_link,
        description: channel_description,
        image: rss_image,
        items,
    };

    let mut db = db.lock().unwrap();

    // save data to DB
    db.save(user_address, rss_channel).await?;
    Ok(())
}
