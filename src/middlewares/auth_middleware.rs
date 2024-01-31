use crate::{ctx::Ctx, models::auth_model::Claims, services::auth_service::cookie_extractor};
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

pub async fn require_auth<B>(ctx: Ctx, mut req: Request<B>, next: Next<B>) -> Response<BoxBody> {
    println!("\n---> My first Ctx content: \n{:?}", ctx);

    let headers = req.headers();
    let cookies = cookie_extractor(headers);

    let token_str = match cookies.get("KRAKEN_AUTH") {
        Some(value) => value,
        None => return (StatusCode::UNAUTHORIZED, "Token doesn't exist").into_response(),
    };

    let decoding_key = DecodingKey::from_secret(dotenv!("JWT_SECRET").as_ref());
    let claims = match decode::<Claims>(&token_str, &decoding_key, &Validation::default()) {
        Ok(value) => value.claims,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Token is invalid").into_response(),
    };

    req.extensions_mut().insert(claims);

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
