#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use command::ListRssArticles;
use opml::OPML;
use std::fmt::Display;

use rss::*;
use serde::Deserialize;
use structopt::StructOpt;

pub mod config;
pub mod db;
pub mod element;
pub mod preprocess;
pub use config::Config;
pub mod command;
pub use command::ApplicationArguments;
pub mod cache;
pub mod ui;

use crate::{db::GLOBAL_DATA, preprocess::process};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let default_config = include_str!("../.rss/RAW.opml");
    let document = Config::from_str(default_config)?;

    let opt = ApplicationArguments::from_args();
    match opt.command {
        command::Command::RunApp => {
            ui::run_ui(&document)?;
        }
        command::Command::Subscribe(_) => println!("subscribe"),
        command::Command::ListRssArticles(ListRssArticles { index }) => {
            if let Some(index) = index {
                let category_len = document.category_len();
                let index = if index == 0 {
                    0
                } else if index >= category_len {
                    category_len - 1
                } else {
                    index - 1
                };
                if let Some(outline) = document.body().outlines.get(index) {
                    for (idx, item) in outline.outlines.iter().enumerate() {
                        println!("🎈{}: {}", idx, item.title.clone().unwrap_or_default());
                    }
                }
            } else {
                document.body().outlines.iter().for_each(|value| {
                    for (idx, item) in value.outlines.iter().enumerate() {
                        println!("🎈{}: {}", idx, item.title.clone().unwrap_or_default());
                    }
                });
            }
        }
        command::Command::ReadOneArticle => println!("read one article"),
        command::Command::Category => {
            let _rss_category = document
                .body()
                .outlines
                .iter()
                .map(|value| {
                    println!("{}", value.title.clone().unwrap_or_default());
                })
                .collect::<Vec<_>>();
        }
    }

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
