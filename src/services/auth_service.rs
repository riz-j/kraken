use axum::http::{header, HeaderMap};
use std::collections::HashMap;
use tower_cookies::Cookie;

pub fn cookie_extractor(headers: &HeaderMap) -> HashMap<String, String> {
    let mut cookies_map = HashMap::new();

    let cookies_in_header = match headers.get(header::COOKIE) {
        Some(value) => value,
        None => return cookies_map,
    };

    let cookie_str = match cookies_in_header.to_str() {
        Ok(str) => str,
        Err(_) => return cookies_map,
    };

    cookie_str
        .split(';')
        .filter_map(|s| Cookie::parse_encoded(s).ok())
        .for_each(|cookie| {
            cookies_map.insert(cookie.name().to_string(), cookie.value().to_string());
        });

    cookies_map
}
