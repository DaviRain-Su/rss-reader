use rss::*;
use std::fs;
pub mod db;
pub mod element;
pub mod preprocess;

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

    for item in rss_channel.items.iter() {
        println!("🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈");
        let link = item.link.clone().unwrap_or_default();
        let response = reqwest::get(link).await?.text().await?;
        let dom = tl::parse(&response, tl::ParserOptions::new().track_ids()).unwrap();
        // let parser = dom.parser();

        // parse mirror title
        let handle = dom
            .query_selector("title")
            .and_then(|mut iter| iter.next())
            .unwrap();
        let node = handle.get(dom.parser()).unwrap();

        println!("{}", node.inner_text(dom.parser()));

        let _ = dom.query_selector("p").map(|mut iter| loop {
            if let Some(handle) = iter.next() {
                let node = handle.get(dom.parser()).unwrap();
                println!("{}", node.inner_text(dom.parser()));
            } else {
                break;
            }
        });
    }
    // println!("🎈🎈🎈🎈🎈🎈🎈{:#?}", parser);
    // fs::write("guoyu.html", format!("{:#?}", parser).as_bytes())?;
    Ok(())
}
