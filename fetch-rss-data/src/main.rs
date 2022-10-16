use rss::*;
pub mod element;
pub mod preprocess;
pub mod db;
use scraper::{Html, Selector};


use crate::db::GLOBAL_DATA;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // https://guoyu.submirror.xyz
    
    let response = reqwest::get("https://guoyu.submirror.xyz")
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&response[..])?;

    // println!("{:#?}", channel);

    preprocess::process(channel, &GLOBAL_DATA);

    // println!("GLOBAL_DATA: {:#?}", GLOBAL_DATA);

    let tep = GLOBAL_DATA.lock().unwrap();
    let rss_channel = tep.get("davirain.eth".to_owned()).unwrap();
    // println!("{:#?}", rss_channel);

    // for item in rss_channel.items.iter() {
    //     println!("🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈");
    //     let link = item.link.clone().unwrap_or_default();
    //     let response = reqwest::get(link)
    //         .await?
    //         .text()
    //         .await?;
    //     let document = Html::parse_document(&response);
    //     println!("🎈🎈🎈🎈🎈🎈🎈{:#?}", document);
    // }

    println!("🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈");
    // let link = item.link.clone().unwrap_or_default();
    let response = reqwest::get("https://guoyu.mirror.xyz/cj7GHFkl4wW8qk2m9y_tESMHrQ8CeLd9SaU6FhkojyQ")
        .await?
        .text()
        .await?;
    let document = Html::parse_fragment(&response);
    // let body_selector = Selector::parse("body").unwrap();
    // let p_selector = Selector::parse("p").unwrap();
    // for ele in document.select(&body_selector) {
        // println!("{:?}", ele);
    // }
    // for element in div.select(&p_selector) {
    //     println!("{:?}", element);
    // }

    println!("🎈🎈🎈🎈🎈🎈🎈{:#?}", document);
    // let tree = document.tree.clone();
    // println!("{:?}", body_selector);
    // let element = document.

    Ok(())
}