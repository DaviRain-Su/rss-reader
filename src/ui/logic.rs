use rss::Channel;

#[derive(Debug)]
pub struct XmlChannel {
    pub xmlurl: String,
    pub channel: Channel,
}

use crate::{
    db::{preprocess::process, sqlite_db::SqliteDb, titles::Titles, DatabaseReader},
    element::Article,
};

/// get Titles
pub fn get_titles(url: &str) -> anyhow::Result<Titles> {
    let sqlite_db = SqliteDb;

    let rss_titles = sqlite_db.get_articles_titles(url)?;

    Ok(rss_titles)
}

/// run rss main logic to insert data
pub fn run(url: &str) -> anyhow::Result<()> {
    let response = reqwest::blocking::get(url)?.bytes()?;

    let channel = Channel::read_from(&response[..])?;

    let xml_channel = XmlChannel {
        xmlurl: url.to_string(),
        channel,
    };

    let mut sqlite_db = SqliteDb;

    process(xml_channel, &mut sqlite_db)?;

    Ok(())
}

/// run get article
pub fn get_article(url: &str, title: &str) -> anyhow::Result<Option<Article>> {
    let sqlite_db = SqliteDb;

    let rss_titles = sqlite_db.get_article(url, title)?;

    Ok(rss_titles)
}

#[test]
// #[ignore]
fn test_get_titles() {
    let _ = run("https://guoyu.submirror.xyz").unwrap();
    let titles = get_titles("https://guoyu.submirror.xyz").unwrap();
    println!("{:#?}", titles);

    let article = get_article("https://guoyu.submirror.xyz", "永不消逝的哈希 — 郭宇")
        .unwrap()
        .unwrap();
    println!("{}", article);
}
