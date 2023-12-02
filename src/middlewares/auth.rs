use std::collections::HashMap;

use axum::{
    body::BoxBody,
    http::{header, HeaderMap, Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use tower_cookies::Cookie;

pub async fn require_auth<B>(req: Request<B>, next: Next<B>) -> Response<BoxBody> {
    let headers = req.headers();
    let cookies = cookie_extractor(headers);

    println!("{:?}", cookies);

    if cookies.get("KRAKEN_AUTH") != Some(&"my_secret_token".to_string()) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized Access").into_response();
    }

    next.run(req).await
}

fn cookie_extractor(headers: &HeaderMap) -> HashMap<String, String> {
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
