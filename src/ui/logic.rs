use rss::Channel;

#[derive(Debug)]
pub struct XmlChannel {
    pub xmlurl: String,
    pub channel: Channel,
}

use crate::db::{preprocess::process, GLOBAL_DATA, DatabaseReader, titles::Titles};

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

#[test]
// #[ignore]
fn test_get_titles() {
    let ret = run("https://guoyu.submirror.xyz").unwrap();
    let titles = get_titles("https://guoyu.submirror.xyz").unwrap();
    println!("{:#?}", titles);
}
