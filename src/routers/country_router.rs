use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;

use crate::{
    clients::country_client,
    models::country_model::{InsertCountry, SelectCountry, UpdateCountry},
};

async fn get_country_by_id(
    Path(country_id): Path<i64>,
) -> Result<(StatusCode, Json<SelectCountry>), Response> {
    match country_client::select_country(country_id).await {
        Ok(country) => Ok((StatusCode::OK, Json(country))),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Country not found" })),
        )
            .into_response()),
    }
}

async fn list_countries() -> Result<(StatusCode, Json<Vec<SelectCountry>>), Response> {
    match country_client::list_countries().await {
        Ok(countries) => Ok((StatusCode::OK, Json(countries))),
        Err(err) => {
            println!("--> Error listing countries: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error listing countries" })),
            )
                .into_response())
        }
    }
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

async fn delete_country(Path(country_id): Path<i64>) -> Response {
    match country_client::delete_country(country_id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => {
            println!("--> Error deleting country: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error deleting country" })),
            )
                .into_response()
        }
    }
}

pub fn country_router() -> Router {
    Router::new()
        .route("/countries", get(list_countries))
        .route("/countries/:country_id", get(get_country_by_id))
        .route("/countries", post(create_country))
        .route("/countries/:country_id", put(update_country))
        .route("/countries/:country_id", delete(delete_country))
}
