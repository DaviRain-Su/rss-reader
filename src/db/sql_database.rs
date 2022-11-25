use super::titles::Titles;

pub const RSS_DATABSE_NAME: &str = "rss.db";

/// create a rss database
pub fn create_rss_database() -> anyhow::Result<()> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = "
        CREATE TABLE rssopml (title TEXT, description TEXT, htmlurl TEXT, xmlurl TEXT, titles TEXT);
    ";
    connection.execute(query)?;

    Ok(())
}

/// insert item
pub fn insert(
    title: String,
    description: String,
    htmlurl: String,
    xmlurl: String,
    titles: Titles,
) -> anyhow::Result<()> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    // if this xmlurl exist we not insert this items
    if !is_exist(&xmlurl)? {
        // encode titles
        let titles = serde_json::to_string(&titles)?;

        let query = format!(
            "
        INSERT INTO rssopml VALUES ('{}', '{}', '{}', '{}', '{}');
    ",
            title, description, htmlurl, xmlurl, titles
        );

        connection.execute(query)?;
    }

    Ok(())
}

pub fn get_titles(xml: &str) -> anyhow::Result<Titles> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!("SELECT titles FROM rssopml WHERE xmlurl == '{}'", xml);

    let mut result = String::new();
    connection.iterate(query, |pairs| {
        for &(_, value) in pairs.iter() {
            let value = value.unwrap_or_default();
            result = value.to_string();
        }
        true
    })?;

    // decode title
    let titles: Titles = serde_json::from_str(&result)?;

    Ok(titles)
}

pub fn list() -> anyhow::Result<()> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!("SELECT * FROM rssopml");

    connection.iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        true
    })?;

    Ok(())
}

/// is or not exist
pub fn is_exist(xmlurl: &str) -> anyhow::Result<bool> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!("SELECT * FROM rssopml WHERE xmlurl == '{}'", xmlurl);

    let mut result = false;
    connection.iterate(query, |pairs| {
        for &(_, value) in pairs.iter() {
            if value.is_none() {
                result = false;
            } else {
                result = true;
            }
        }
        true
    })?;

    Ok(result)
}

#[test]
fn test_create_db() {
    // create
    // let _ = create_rss_database().unwrap();
    let _ = insert(
        "davirain".into(),
        "hello, world".into(),
        "davirain.xyz".into(),
        "davirain.xyz".into(),
        Titles::default(),
    )
    .unwrap();
    let titles = get_titles("davirain.xyz").unwrap();
    let _ = list().unwrap();
    let ret = is_exist("davirain.xyz").unwrap();
    println!("{}", ret);
    println!("{:?}", titles);
}
