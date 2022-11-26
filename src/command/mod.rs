use anyhow::Result;
use reqwest::Url;
use structopt::StructOpt;

/// Rss Reader Command
#[derive(Debug, StructOpt)]
pub enum Command {
    /// init rss
    Init,
    /// run rss reader app
    #[structopt(name = "run-app")]
    RunApp,
    /// subscribe a new rss source
    #[structopt(name = "subscribe-rss")]
    Subscribe(SubscribeRss),
    /// rss category
    #[structopt(name = "category")]
    Category,
    /// clean database
    #[structopt(name = "clean")]
    Clean,
}

#[derive(Debug, StructOpt)]
pub struct ListRssArticles {
    pub index: Option<usize>,
}

#[derive(Debug, StructOpt)]
pub struct SubscribeRss {
    #[structopt(parse(try_from_str = parse_url))]
    pub url: Option<Url>,
}

fn parse_url(s: &str) -> Result<Url> {
    Ok(s.parse()?)
}

impl SubscribeRss {
    // parse rss url
    // save rss data to databse
    // save rss url to opml file
}
/// Rss Reader app
#[derive(Debug, StructOpt)]
#[structopt(name = "rss-reader")]
pub struct ApplicationArguments {
    #[structopt(subcommand)]
    pub command: Command,
}
