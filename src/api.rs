use crate::{
    mc::ModelController,
    middlewares::auth_middleware::require_ctx,
    routers::api::{city_router::city_router, country_router::country_router},
};
use axum::{middleware, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub fn router(mc: ModelController) -> Router {
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    Router::new()
        .merge(country_router(mc.clone()))
        .merge(city_router(mc.clone()))
        .layer(middleware::from_fn(require_ctx))
        .layer(tracing_layer)
}
