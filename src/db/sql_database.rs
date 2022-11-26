use super::titles::Titles;

pub const RSS_DATABSE_NAME: &str = "rss.db";
pub const RSS_TABLE: &str = "rssopml";

/// create a rss database
pub fn create_rss_database() -> anyhow::Result<()> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!("
        CREATE TABLE {} (title TEXT, description TEXT, htmlurl TEXT, xmlurl TEXT, articlestitles TEXT);
    ", RSS_TABLE);
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
        INSERT INTO {} VALUES ('{}', '{}', '{}', '{}', '{}');
    ",
            RSS_TABLE, title, description, htmlurl, xmlurl, titles
        );

        connection.execute(query)?;
    }

    Ok(())
}

/// get rss title
pub fn get_rss_title(xml: &str) -> anyhow::Result<String> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!("SELECT title FROM {} WHERE xmlurl == '{}'", RSS_TABLE, xml);

    let mut result = String::new();
    connection.iterate(query, |pairs| {
        for &(_, value) in pairs.iter() {
            let value = value.unwrap_or_default();
            result = value.to_string();
        }
        true
    })?;

    Ok(result)
}

/// get description
pub fn get_description(xml: &str) -> anyhow::Result<String> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!(
        "SELECT description FROM {} WHERE xmlurl == '{}'",
        RSS_TABLE, xml
    );

    let mut result = String::new();
    connection.iterate(query, |pairs| {
        for &(_, value) in pairs.iter() {
            let value = value.unwrap_or_default();
            result = value.to_string();
        }
        true
    })?;

    Ok(result)
}

/// get article html url
pub fn get_htmlurl(xml: &str) -> anyhow::Result<String> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!(
        "SELECT htmlurl FROM {} WHERE xmlurl == '{}'",
        RSS_TABLE, xml
    );

    let mut result = String::new();
    connection.iterate(query, |pairs| {
        for &(_, value) in pairs.iter() {
            let value = value.unwrap_or_default();
            result = value.to_string();
        }
        true
    })?;

    Ok(result)
}

/// get article titles
pub fn get_article_titles(xml: &str) -> anyhow::Result<Titles> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!(
        "SELECT articlestitles FROM {} WHERE xmlurl == '{}'",
        RSS_TABLE, xml
    );

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

/// update articles titles
pub fn update_articles_titles(xml: &str, titles: Titles) -> anyhow::Result<()> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    // encode titles
    let titles = serde_json::to_string(&titles)?;

    let query = format!(
        "
          UPDATE {} SET articlestitles = '{}' WHERE xmlurl == '{}';
  ",
        RSS_TABLE, titles, xml
    );

    connection.execute(query)?;

    todo!()
}

/// list table rss opml
pub fn list() -> anyhow::Result<()> {
    let connection = sqlite::open(RSS_DATABSE_NAME)?;

    let query = format!("SELECT * FROM {}", RSS_TABLE);

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

    let query = format!("SELECT * FROM {} WHERE xmlurl == '{}'", RSS_TABLE, xmlurl);

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
#[ignore]
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
    let titles = get_article_titles("davirain.xyz").unwrap();
    println!("article titles: {:#?}", titles);
    let rss_tile = get_rss_title("davirain.xyz").unwrap();
    println!("rss tile: {:#?}", rss_tile);
    let description = get_description("davirain.xyz").unwrap();
    println!("description: {:#?}", description);
    let htmlurl = get_htmlurl("davirain.xyz").unwrap();
    println!("htmlurl: {:#?}", htmlurl);
    // let _ = list().unwrap();
    let ret = is_exist("davirain.xyz").unwrap();
    println!("1{}", ret);
}
