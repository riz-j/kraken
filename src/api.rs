use crate::{
    middlewares::auth_middleware::require_auth,
    routers::{city_router::city_router, country_router::country_router},
};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(country_router())
        .merge(city_router())
        .layer(axum::middleware::from_fn(require_auth))
    // .layer(axum::middleware::from_fn(print_country_id))
}
