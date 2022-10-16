use rss::*;
use std::fs;
pub mod db;
pub mod element;
pub mod preprocess;

use crate::db::GLOBAL_DATA;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // https://guoyu.submirror.xyz
    // https://submirror.xyz/0xA15f95B1BD801BFd67E769584F65cF15add56b6F

    let response = reqwest::get("https://guoyu.submirror.xyz")
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&response[..])?;

    // println!("{:#?}", channel);

    preprocess::process("davirain.eth".to_string(), channel, &GLOBAL_DATA).await?;

    // println!("GLOBAL_DATA: {:#?}", GLOBAL_DATA);

    let tep = GLOBAL_DATA.lock().unwrap();
    let rss_articles = tep
        .get_rss_articles("davirain.eth".to_owned(), "guoyu.eth".to_string())
        .unwrap();
    // let rss_articles = tep
    //     .get_rss_articles("davirain.eth".to_owned(), "0xA15f95B1BD801BFd67E769584F65cF15add56b6F".to_string())
    //     .unwrap();

    println!("{}", rss_articles);
    Ok(())
}
