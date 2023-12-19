use axum::{routing::get_service, Router};
use tower_http::services::{ServeDir, ServeFile};

pub fn office_router() -> Router {
    Router::new()
        .route(
            "/office",
            get_service(ServeFile::new("./frontend-main/dist/index.html")),
        )
        .nest_service(
            "/office/assets",
            ServeDir::new("./frontend-main/dist/assets"),
        )
        .route(
            "/office/*path",
            get_service(ServeFile::new("./frontend-main/dist/index.html")),
        )
}
