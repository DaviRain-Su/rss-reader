const NOSQL_DATABASE: &str = ".rss_reader/RSS_NOSQL";
const SIMPLE_MAP_KEY: &str = "rss";
use simpledb::Database;

pub fn open_database_with_path(path: &str) -> Database {
    let db = Database::open(path).unwrap();
    println!("open database: {}", db.path);
    db
}

// key is hash(rssxmlurl + title), value is encode articles string
/// insert data
pub fn insert(key: &str, value: &[u8]) -> anyhow::Result<()> {
    let db = open_database_with_path(NOSQL_DATABASE);
    db.map_put(SIMPLE_MAP_KEY, key, value)?;

    Ok(())
}

/// get data
pub fn get(key: &str) -> anyhow::Result<Vec<u8>> {
    let db = open_database_with_path(NOSQL_DATABASE);
    let r = db
        .map_get(SIMPLE_MAP_KEY, key)?
        .ok_or(anyhow::anyhow!("Canot get anything!"))?;

    Ok(r)
}

#[test]
fn test_no_sql() {
    fn get_byte_slice<T: AsRef<[u8]>>(source: &'_ T) -> &'_ [u8] {
        source.as_ref()
    }

    let r = insert("k1", b"v1111").unwrap();

    let ret = get("k1").unwrap();
    println!("{:?}", ret);

    assert_eq!(get_byte_slice(&ret), b"v1111");
}
