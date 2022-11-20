use structopt::StructOpt;
use anyhow::Result;
use reqwest::Url;

/// Rss Reader Command
#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "subscribe-rss")]
    Subscribe(SubscribeRss),
    #[structopt(name = "list-rss-articles")]
    ListRssArticles,
    #[structopt(name = "read-one-article")]
    ReadOneArticle,
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