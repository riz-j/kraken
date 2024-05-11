use axum::routing::get_service;
use axum::Router;
use kraken::mc::ModelController;
use kraken::routers::auth_router::auth_router;
use kraken::routers::spa_router;
use std::error::Error;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

fn static_router() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    kraken::db::init_pool().await;

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let mc = ModelController::new();

    let app = Router::new()
        .nest("/api", kraken::api::router(mc.clone()))
        .nest("/pages", kraken::pages::router(mc.clone()))
        .nest("/rpc", kraken::routers::rpc::rpc_router::router(mc.clone()))
        .layer(CorsLayer::permissive())
        .merge(auth_router())
        .merge(spa_router::office_router())
        .fallback_service(static_router());

    axum::Server::bind(&"0.0.0.0:2900".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
