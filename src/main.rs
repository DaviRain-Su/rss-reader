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
        command::Command::RunApp => {
            // spawn 2 thread run logic
            // first is load data
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
