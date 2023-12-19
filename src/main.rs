use axum::body::Body;
use axum::extract::{path, Path};
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{Router, ServiceExt};
use kraken::middlewares::auth_middleware::{print_country_id, require_auth};
use kraken::routers::auth_router::auth_router;
use kraken::routers::city_router::city_router;
use kraken::routers::country_router::country_router;
use std::error::Error;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

fn static_router() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}

// fn office_assets(Path(filename): Path<String>) -> ServeDir {
//     let fileserve = ServeFile::new("./frontend-main/dist/index.html").into();
//     fileserve
// }

fn frontend_router() -> Router {
    Router::new()
        // .route("/", get_service(ServeDir::new("./frontend-main/dist")))
        // .route("/:path", get_service(ServeDir::new("./frontend-main/dist")))
        .route(
            "/office",
            get_service(ServeFile::new("./frontend-main/dist/index.html")),
        )
        // .merge(office_router())
        .nest_service(
            "/office/assets",
            ServeDir::new("./frontend-main/dist/assets"),
        )
        // .route(
        //     "/office/assets/index-EnuhBH9F.css",
        //     get_service(ServeFile::new(
        //         "./frontend-main/dist/assets/index-EnuhBH9F.css",
        //     )),
        // )
        .route(
            "/office/*path",
            get_service(ServeFile::new("./frontend-main/dist/index.html")),
        )
    // .fallback(get_service(ServeDir::new("./frontend-main/dist")))
    // .fallback(get_service(ServeDir::new("./frontend-main/dist")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    kraken::db::init_pool().await;

    let app = Router::new()
        .merge(country_router())
        .merge(city_router())
        .layer(CorsLayer::permissive())
        // .layer(axum::middleware::from_fn(require_auth))
        .layer(axum::middleware::from_fn(print_country_id))
        .merge(auth_router())
        // .route(
        //     "/assets",
        //     get_service(ServeDir::new("./frontend-main/dist/assets")),
        // )
        .merge(frontend_router())
        .fallback_service(get_service(ServeDir::new("./frontend-main/dist")));
    // .nest("/office", frontend_router());
    // .fallback_service(frontend_router());

    axum::Server::bind(&"0.0.0.0:2900".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
