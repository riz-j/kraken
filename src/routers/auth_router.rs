use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use cookie::Cookie;
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::auth_model::Claims;

async fn login() -> impl IntoResponse {
    let encoding_key = EncodingKey::from_secret(dotenv!("JWT_SECRET").as_ref());
    let timestamp_now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        user_id: 123123,
        iat: timestamp_now,
        exp: timestamp_now + (30),
    };

    let token = encode(&Header::default(), &claims, &encoding_key).unwrap();

    let mut cookie = Cookie::new("KRAKEN_AUTH", token);
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
