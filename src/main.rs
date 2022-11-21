#![allow(unused_assignments)]

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
