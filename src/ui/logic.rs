use rss::Channel;

#[derive(Debug)]
pub struct XmlChannel {
    pub xmlurl: String,
    pub channel: Channel,
}

use crate::{db::{preprocess::process, titles::Titles, DatabaseReader, GLOBAL_DATA}, element::Article};

/// get Titles
pub fn get_titles(url: &str) -> anyhow::Result<Titles> {
    let tep = GLOBAL_DATA.lock().unwrap();

    let rss_titles = tep.get_articles_titles(url)?;

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

    process(xml_channel, &GLOBAL_DATA)?;

    Ok(())
}


pub fn get_article(url: &str, title: &str) -> anyhow::Result<Option<Article>> {
    let tep = GLOBAL_DATA.lock().unwrap();

    let rss_titles = tep.get_article(url, title)?;

    Ok(rss_titles.cloned())
}

#[test]
// #[ignore]
fn test_get_titles() {
    let ret = run("https://guoyu.submirror.xyz").unwrap();
    let titles = get_titles("https://guoyu.submirror.xyz").unwrap();
    println!("{:#?}", titles);

    let article = get_article("https://guoyu.submirror.xyz", "永不消逝的哈希 — 郭宇").unwrap().unwrap();
    println!("{}", article);
}
