use crate::ctx::{Ctx, MyError};
use axum::{
    body::BoxBody,
    extract::Path,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::collections::HashMap;

pub async fn require_auth<B>(
    ctx: Result<Ctx, MyError>,
    req: Request<B>,
    next: Next<B>,
) -> Response<BoxBody> {
    if ctx.is_err() {
        return (StatusCode::UNAUTHORIZED, "Invalid Auth").into_response();
    }

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
