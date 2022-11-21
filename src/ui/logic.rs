use rss::Channel;

use crate::{db::GLOBAL_DATA, preprocess::process, utils::get_author_address_or_name};

/// get Titles
pub async fn get_titles(url: &str) -> anyhow::Result<Vec<String>> {
    let response = reqwest::get(url).await?.bytes().await?;

    let channel = Channel::read_from(&response[..])?;

    process(channel, &GLOBAL_DATA).await?;

    let tep = GLOBAL_DATA.lock().unwrap();

    let author_address = get_author_address_or_name(url);

    let rss_titles = tep.get_rss_titles(author_address)?;

    Ok(rss_titles)
}
