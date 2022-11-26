#![allow(unused_assignments)]

use structopt::StructOpt;
pub mod config;
pub mod db;
pub mod element;
pub use config::Config;
pub mod command;
pub use command::ApplicationArguments;
pub mod cache;
pub mod ui;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let default_config = include_str!("../.rss/RAW.opml");
    let document = Config::from_str(default_config)?;

    let opt = ApplicationArguments::from_args();
    match opt.command {
        command::Command::Init => {
            // first is load data
            let document1 = document.clone();

            let load_database_task = tokio::spawn(async move {
                let category_len = document1.category_len();
                for idx in 0..category_len {
                    for item in document1.outlines(idx) {
                        let rss_url = item.rss_url.clone();
                        match ui::logic::run(&rss_url).await {
                            Ok(_) => println!("Run logic successful!"),
                            Err(err) => println!("{:?}", err),
                        }
                    }
                }
            });

            match load_database_task.await {
                Ok(_) => println!("SUCCESS!"),
                Err(e) => println!("Esixt failed{:?}", e),
            }
        }
        command::Command::RunApp => {
            // spawn 2 thread run logic

            // second run ui
            let task = tokio::spawn(async move { ui::run_ui(&document) });

            match task.await.unwrap() {
                Ok(_) => println!("SUCCESS!"),
                Err(e) => println!("{:?}", e),
            }
        }
        command::Command::Subscribe(_) => println!("subscribe"),
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
        command::Command::Clean => println!("clean databae!"),
    }

    Ok(())
}
