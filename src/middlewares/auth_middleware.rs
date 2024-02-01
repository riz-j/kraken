use crate::ctx::{Ctx, CtxError};
use axum::{
    body::BoxBody,
    extract::Path,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::collections::HashMap;

pub async fn require_auth<B>(
    ctx: Result<Ctx, CtxError>,
    req: Request<B>,
    next: Next<B>,
) -> Response<BoxBody> {
    if ctx.is_err() {
        let response = axum::response::Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(
                "Set-Cookie",
                "KRAKEN_AUTH=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT",
            )
            .body(axum::body::Full::from("Invalid Auth"))
            .unwrap();

        return response.into_response();
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
