use crate::{
    mc::ModelController,
    middlewares::auth_middleware::require_auth,
    routers::{city_router::city_router, country_router::country_router},
};
use axum::Router;

pub fn router(mc: ModelController) -> Router {
    Router::new()
        .merge(country_router(mc.clone()))
        .merge(city_router())
        .layer(axum::middleware::from_fn(require_auth))
}
