#![allow(unused_assignments)]

use command::ListRssArticles;
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
pub mod utils;

fn main() -> anyhow::Result<()> {
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

    Ok(())
}
