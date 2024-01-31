use crate::services::auth_service::cookie_extractor;
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    RequestPartsExt,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Ctx {
    auth_cookie: String,
}

impl Ctx {
    pub fn new(auth_cookie: String) -> Self {
        Self { auth_cookie }
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = MyError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, MyError> {
        let headers = parts.extract::<HeaderMap>().await.unwrap();

        let cookies = cookie_extractor(&headers);

        let token_str = cookies.get("KRAKEN_AUTH").unwrap().clone();

        Ok(Ctx::new(token_str))
    }
}

pub struct MyError;

impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap()
            .into_response()
    }
}
