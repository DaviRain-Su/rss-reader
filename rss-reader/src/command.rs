use anyhow::Result;
use reqwest::Url;
use structopt::StructOpt;

/// Rss Reader Command
#[derive(Debug, StructOpt)]
pub enum Command {
    /// run rss reader app
    #[structopt(name = "run-app")]
    RunApp,
    /// subscribe single rss source
    #[structopt(name = "subscribe-rss")]
    Subscribe(SubscribeRss),
    /// list rss articles
    #[structopt(name = "list-rss-articles")]
    ListRssArticles(ListRssArticles),
    /// read one article
    #[structopt(name = "read-one-article")]
    ReadOneArticle,
    /// rss category
    #[structopt(name = "category")]
    Category,
}

#[derive(Debug, StructOpt)]
pub struct ListRssArticles {
    pub index: Option<usize>,
}

#[derive(Debug, StructOpt)]
pub struct SubscribeRss {
    #[structopt(parse(try_from_str = parse_url))]
    url: Option<Url>,
}

fn parse_url(s: &str) -> Result<Url> {
    Ok(s.parse()?)
}

/// Rss Reader app
#[derive(Debug, StructOpt)]
#[structopt(name = "rss-reader")]
pub struct ApplicationArguments {
    #[structopt(subcommand)]
    pub command: Command,
}
