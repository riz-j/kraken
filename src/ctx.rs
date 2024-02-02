use crate::{
    mc::ModelController,
    models::{auth_model::Claims, user_model::UserSelect},
    services::auth_service::cookie_extractor,
};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    RequestPartsExt,
};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Ctx {
    auth_cookie: String,
    claims: Claims,
}

impl Ctx {
    pub fn new(auth_cookie: String, claims: Claims) -> Self {
        Self {
            auth_cookie,
            claims,
        }
    }

    pub fn get_auth_cookie(&self) -> String {
        self.auth_cookie.clone()
    }

    pub async fn get_user(&self) -> UserSelect {
        let mc = ModelController::new();
        mc.user_store
            .select(&self, self.claims.user_id)
            .await
            .unwrap()
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = CtxError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, CtxError> {
        let headers = parts.extract::<HeaderMap>().await.unwrap();

        let cookies = cookie_extractor(&headers);

        let token_str = match cookies.get("KRAKEN_AUTH") {
            Some(value) => value.clone(),
            None => return Err(CtxError),
        };

        let decoding_key = DecodingKey::from_secret(dotenv!("JWT_SECRET").as_ref());

        let claims = match decode::<Claims>(&token_str, &decoding_key, &Validation::default()) {
            Ok(value) => value.claims,
            Err(_) => return Err(CtxError),
        };

        Ok(Ctx::new(token_str, claims))
    }
}

#[derive(Debug)]
pub struct CtxError;

impl IntoResponse for CtxError {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap()
            .into_response()
    }
}
