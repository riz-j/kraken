use crate::{
    models::{auth_model::Claims, user_model::UserId},
    services::auth_service::cookie_extractor,
};
use axum::{
    body::BoxBody,
    extract::Path,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::collections::HashMap;

pub async fn require_auth<B>(mut req: Request<B>, next: Next<B>) -> Response<BoxBody> {
    let headers = req.headers();
    let cookies = cookie_extractor(headers);

    let decoding_key = DecodingKey::from_secret(dotenv!("JWT_SECRET").as_ref());
    let token_str = match cookies.get("KRAKEN_AUTH") {
        Some(value) => value,
        None => return (StatusCode::UNAUTHORIZED, "Token doesn't exist").into_response(),
    };

    let token = match decode::<Claims>(&token_str, &decoding_key, &Validation::default()) {
        Ok(value) => value,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Token is invalid").into_response(),
    };

    let claims = token.claims;

    let timestamp_now = chrono::Utc::now().timestamp() as usize;
    if claims.exp < timestamp_now {
        return (StatusCode::UNAUTHORIZED, "Token is expired").into_response();
    }

    req.extensions_mut().insert(UserId { id: claims.user_id });

    next.run(req).await
}

pub async fn print_country_id<B>(
    Path(params): Path<HashMap<String, String>>,
    req: Request<B>,
    next: Next<B>,
) -> Response<BoxBody> {
    let country_id = match params.get("country_id") {
        Some(value) => value,
        None => "nothing",
    };

    println!("Country ID: {}", country_id);

    next.run(req).await
}
