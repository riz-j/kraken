use axum::{routing::get_service, Router};
use tower_http::services::{ServeDir, ServeFile};

pub fn office_router() -> Router {
    Router::new()
        .route(
            "/office",
            get_service(ServeFile::new("./frontend-office/dist/index.html")),
        )
        .nest_service(
            "/office/assets",
            ServeDir::new("./frontend-office/dist/assets"),
        )
        .route(
            "/office/*path",
            get_service(ServeFile::new("./frontend-office/dist/index.html")),
        )
}
