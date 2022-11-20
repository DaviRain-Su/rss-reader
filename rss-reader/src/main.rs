#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::fmt::Display;
use opml::OPML;

use rss::*;
use serde::Deserialize;
use structopt::StructOpt;

pub mod db;
pub mod element;
pub mod preprocess;
pub mod config;
pub use config::Config;
pub mod command;
pub use command::ApplicationArguments;

use crate::{db::GLOBAL_DATA, preprocess::process};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let default_config = include_str!("../.rss/RAW.opml");
    let document = OPML::from_str(default_config)?;
    println!("{:#?}", document);

    // let opt = ApplicationArguments::from_args();
    // println!("{:?}", opt);
    // match opt.command {
    //     Command::Subscribe(SubscribeRss { url }) => {
    //         println!("subscribe url: {:#?}", url);

    //         let url = if let Some(val) = url {
    //             val
    //         } else {
    //             panic!("Invalid url");
    //         };

    //         let response = reqwest::get(url)
    //             .await?
    //             .bytes()
    //             .await?;

    //         let channel = Channel::read_from(&response[..])?;
    //         process("davirain.eth".to_string(), channel, &GLOBAL_DATA).await?;

    //         let tep = GLOBAL_DATA.lock().unwrap();
    //         let rss_articles = tep
    //             .get_rss_articles("davirain.eth".to_owned(), "guoyu.eth".to_string())
    //             .unwrap();

    //         println!("{}", rss_articles);
    //     }
    //     Command::ListRssArticles => todo!(),
    //     Command::ReadOneArticle => todo!(),
    // }

    Ok(())
}
