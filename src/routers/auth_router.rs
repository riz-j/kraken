use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use cookie::Cookie;

async fn login() -> impl IntoResponse {
    let mut cookie = Cookie::new("KRAKEN_AUTH", "my_secret_token");
    cookie.set_secure(true);
    cookie.set_domain("localhost");
    cookie.set_path("/");
    cookie.set_http_only(true);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Set-Cookie", cookie.to_string())
        .body("Cookies set!".to_string())
        .unwrap();

    response
}

pub fn auth_router() -> Router {
    Router::new().route("/auth/login", post(login))
}
