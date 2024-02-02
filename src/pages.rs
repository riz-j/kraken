use crate::{
    mc::ModelController, middlewares::auth_middleware::require_ctx,
    routers::askama_router::askama_router,
};
use axum::{middleware, Router};

pub fn router(mc: ModelController) -> Router {
    Router::new()
        .merge(askama_router(mc.clone()))
        .layer(middleware::from_fn(require_ctx))
}
