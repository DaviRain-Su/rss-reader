use rss::Channel;

#[derive(Debug)]
pub struct XmlChannel {
    pub xmlurl: String,
    pub channel: Channel,
}

use crate::db::{preprocess::process, GLOBAL_DATA};

/// get Titles
pub fn get_titles(url: &str) -> anyhow::Result<Vec<String>> {
    let response = reqwest::blocking::get(url)?.bytes()?;

    let channel = Channel::read_from(&response[..])?;
    // println!("channel: {:?}", channel);

    let xml_channel = XmlChannel {
        xmlurl: url.to_string(),
        channel,
    };

    process(xml_channel, &GLOBAL_DATA)?;

    let tep = GLOBAL_DATA.lock().unwrap();

    let rss_titles = tep.get_rss_titles(url.to_string())?;

    Ok(rss_titles)
}

#[test]
#[ignore]
fn test_get_titles() {
    let titles = get_titles("https://guoyu.submirror.xyz").unwrap();
    println!("{:#?}", titles);
}
