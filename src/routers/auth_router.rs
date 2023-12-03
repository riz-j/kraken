use crate::{
    models::{
        auth_model::{Claims, LoginRequest},
        user_model::InsertUser,
    },
    stores::{auth_store, user_store},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use cookie::Cookie;
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    let user = match auth_store::login(&payload).await {
        Ok(value) => value,
        Err(error) => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(format!("Email and/or password is incorrect: {:?}", error))
                .unwrap();
        }
    };

    let encoding_key = EncodingKey::from_secret(dotenv!("JWT_SECRET").as_ref());
    let timestamp_now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        user_id: user.id,
        iat: timestamp_now,
        exp: timestamp_now + (2 * 60),
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

async fn signup(Json(payload): Json<InsertUser>) -> impl IntoResponse {
    let user_id = user_store::insert_user(&payload).await.unwrap();

    (StatusCode::CREATED, Json(json!({ "user_id": user_id.id })))
}

pub fn auth_router() -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/signup", post(signup))
}
