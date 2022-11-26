#![allow(unused_assignments)]

use structopt::StructOpt;
pub mod config;
pub mod db;
pub mod element;
pub use config::Config;
pub mod command;
pub use command::ApplicationArguments;
use tokio::runtime::Runtime;
pub mod cache;
pub mod ui;
pub mod utils;

fn main() -> anyhow::Result<()> {
    let default_config = include_str!("../.rss/saimple_raw.opml");
    let document = Config::from_str(default_config)?;
    println!("ducument: {:#?}", document);
    let rss_urls_len = document.get_all_rss_url().len();
    let cpu_nums = num_cpus::get();
    println!("rss url len: {:?}", rss_urls_len);
    println!("cpu nums: {:?}", cpu_nums);

    let opt = ApplicationArguments::from_args();
    match opt.command {
        command::Command::Init => {
            let runtime = Runtime::new()?;

            runtime.block_on(async {
                // first is load data
                let document1 = document.clone();
                let every_cpu_works = rss_urls_len / cpu_nums;

                let mut thread_tasks = Vec::new();

                for thread_task in 0..cpu_nums {
                    let rss_urls = document1.clone().get_all_rss_url();

                    let load_database_task = tokio::spawn(async move {
                        let start = thread_task * every_cpu_works;
                        let end = if start + every_cpu_works > rss_urls_len {
                            rss_urls_len
                        } else {
                            start + every_cpu_works
                        };
                        for rss_url in &rss_urls[start..end] {
                            match ui::logic::run(&rss_url).await {
                                Ok(_) => println!("Run logic successful!"),
                                Err(err) => println!("{:?}", err),
                            }
                        }
                    });
                    thread_tasks.push(load_database_task);
                }

                for thread_task in thread_tasks {
                    match thread_task.await {
                        Ok(_) => println!("SUCCESS!"),
                        Err(e) => println!("Esixt failed{:?}", e),
                    }
                }
            });
        }
        command::Command::RunApp => {
            // spawn 2 thread run logic

            // second run ui
            let task = std::thread::spawn(move || ui::run_ui(&document));

            match task.join() {
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
