use rss::*;
pub mod element;
pub mod preprocess;
pub mod db;

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

    println!("GLOBAL_DATA: {:#?}", GLOBAL_DATA);

    Ok(())
}