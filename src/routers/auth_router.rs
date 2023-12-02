use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use cookie::Cookie;
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i64,
    iat: usize,
    exp: usize,
}

async fn jwt() -> impl IntoResponse {
    let secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_ref());

    let timestamp_now = chrono::Utc::now().timestamp() as usize;
    let my_claims = Claims {
        user_id: 123123,
        iat: timestamp_now,
        exp: timestamp_now + (7 * 24 * 60 * 60),
    };

    let token = encode(&Header::default(), &my_claims, &key);

    token.unwrap()
}

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
    Router::new().route("/auth/login", post(login).get(jwt))
}
