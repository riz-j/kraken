use axum::routing::get_service;
use axum::Router;
use kraken::routers::askama_router::askama_router;
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

    let app = Router::new()
        .nest("/api", kraken::api::router())
        .merge(askama_router())
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
