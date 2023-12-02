use axum::{
    body::BoxBody,
    http::{header, Request, Response},
    middleware::Next,
};
use tower_cookies::Cookie;

pub async fn require_auth<B>(req: Request<B>, next: Next<B>) -> Response<BoxBody> {
    // Access the request headers
    let headers = req.headers();

    // Try to extract and parse the Cookie header
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Parse the Cookie header to get individual cookies
            let cookies = cookie_str
                .split(';')
                .filter_map(|s| Cookie::parse_encoded(s).ok());

            // Now you can work with the cookies
            for cookie in cookies {
                println!("Cookie: {} = {}", cookie.name(), cookie.value());
            }
        }
    }

    // Proceed with the next middleware or handler
    println!("Logged yo!");
    next.run(req).await
}
