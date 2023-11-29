use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::{
    clients::country_client,
    models::country_model::{InsertCountry, SelectCountry, UpdateCountry},
    schemas::country_schema::CountryExtendedSchema,
};

async fn get_country_by_id(Path(country_id): Path<i64>) -> Json<CountryExtendedSchema> {
    let country = country_client::select_country(country_id).await.unwrap();

    Json(country.into_extended_schema().await)
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

async fn create_country(Json(payload): Json<InsertCountry>) -> Result<StatusCode, Response> {
    match country_client::insert_country(&payload).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            println!("--> Error creating country: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error creating country" })),
            )
                .into_response())
        }
    }
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
        .route("/countries", get(list_countries).post(create_country))
        .route(
            "/countries/:country_id",
            get(get_country_by_id)
                .put(update_country)
                .delete(delete_country),
        )
}