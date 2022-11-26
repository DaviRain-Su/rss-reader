use crate::element::{RssChannel, RssImage, RssItem};
use crate::ui::logic::XmlChannel;

use super::{DatabaseKeeper, DatabaseReader};

pub async fn process<
    Databae: DatabaseReader<Error = anyhow::Error>
        + DatabaseKeeper<Error = anyhow::Error>
        + std::marker::Send,
>(
    xml_channel: XmlChannel,
    databse: &mut Databae,
) -> anyhow::Result<()> {
    let channel_title = xml_channel.channel.title.clone();
    let channel_html_url = xml_channel.channel.link.clone();
    let channel_description = xml_channel.channel.description.clone();

    let rss_image = if let Some(image) = xml_channel.channel.image.clone() {
        Some(RssImage {
            image_name: image.title.clone(),
            image_url: image.url.clone(),
        })
    } else {
        None
    };

    let items = xml_channel
        .channel
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
        channel_html_url,
        description: channel_description,
        image: rss_image,
        items,
        channel_xml_url: xml_channel.xmlurl.clone(),
    };

    // save data to DB need relplace by database
    databse.save(rss_channel).await?;

    Ok(())
}
