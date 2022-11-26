/// get author address from mirror address or rss author name
pub fn get_author_address_or_name(rss_url: &str) -> String {
    // get author address from mirror address or rss author name
    let (_, author_address) = rss_url.rsplit_once("/").unwrap_or_default();

    let mut new_author_address = String::new();

    if author_address.contains(".mirror.xyz") {
        new_author_address = rss_url
            .split_once(".mirror.xyz")
            .unwrap_or_default()
            .0
            .split_once("//")
            .unwrap_or_default()
            .1
            .to_string();

        new_author_address.push_str(".eth");
    } else if author_address.contains(".submirror.xyz") {
        new_author_address = rss_url
            .split_once(".submirror.xyz")
            .unwrap_or_default()
            .0
            .split_once("//")
            .unwrap_or_default()
            .1
            .to_string();
        new_author_address.push_str(".eth");
    } else {
        new_author_address = author_address.to_string();
    }

    new_author_address
}

// https://guoyu.submirror.xyz
pub fn generate_mirror_url(url: &str) -> String {
    
    todo!()
}