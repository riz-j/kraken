use axum::routing::get_service;
use axum::Router;
use kraken::middlewares::auth::require_auth;
use kraken::routers::auth_router::auth_router;
use kraken::routers::city_router::city_router;
use kraken::routers::country_router::country_router;
use std::error::Error;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    kraken::db::init_pool().await;

    let app = Router::new()
        .merge(country_router())
        .merge(city_router())
        .layer(CorsLayer::permissive())
        .layer(axum::middleware::from_fn(require_auth))
        .merge(auth_router())
        .fallback_service(routes_static());

    axum::Server::bind(&"0.0.0.0:2900".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
