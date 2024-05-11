use crate::mc::ModelController;
use crate::middlewares::auth_middleware::require_ctx;
use crate::routers::rpc::rpc_router;
use axum::{middleware, Router};

pub fn router(mc: ModelController) -> Router {
    Router::new()
        .merge(rpc_router::router(mc.clone()))
        .layer(middleware::from_fn(require_ctx))
}
