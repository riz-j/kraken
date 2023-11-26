use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    clients::country_client,
    models::country_model::{InsertCountry, SelectCountry, UpdateCountry},
};

async fn get_country_by_id(Path(country_id): Path<i64>) -> Json<SelectCountry> {
    let country = country_client::select_country(country_id).await.unwrap();

    Json(country)
}

async fn list_countries() -> Json<Vec<SelectCountry>> {
    let countries = country_client::list_countries().await.unwrap();

    Json(countries)
}

async fn create_country(Json(payload): Json<InsertCountry>) -> StatusCode {
    country_client::insert_country(&payload).await.unwrap();

    StatusCode::CREATED
}

async fn update_country(
    Path(country_id): Path<i64>,
    Json(payload): Json<UpdateCountry>,
) -> StatusCode {
    country_client::update_country(country_id, &payload)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}

pub fn country_router() -> Router {
    Router::new()
        .route("/countries", get(list_countries))
        .route("/countries/:country_id", get(get_country_by_id))
        .route("/countries", post(create_country))
        .route("/countries/:country_id", put(update_country))
}
