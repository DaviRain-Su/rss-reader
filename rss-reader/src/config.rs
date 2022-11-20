use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    wallet: RawOpmlConfig,
}


#[derive(Debug, Deserialize)]
struct RawOpmlConfig {
    rss_source_file: Option<String>,
}