use crate::{
    mc::ModelController,
    schemas::city_schema::{CityExtendedSchema, CitySummarizedSchema},
    stores::city_store,
};
use axum::{extract::Path, routing::get, Json, Router};

async fn get_city_by_id(Path(city_id): Path<i64>) -> Json<CityExtendedSchema> {
    let city = city_store::select_city(city_id).await.unwrap();

    Json(city.into_extended_schema().await)
}

async fn list_cities() -> Json<Vec<CitySummarizedSchema>> {
    let cities = city_store::list_cities().await.unwrap();
    let cities_summarized = cities
        .iter()
        .map(|city| city.into_summarized_schema())
        .collect();

    Json(cities_summarized)
}

pub fn city_router(mc: ModelController) -> Router {
    Router::new()
        .route("/cities", get(list_cities))
        .route("/cities/:city_id", get(get_city_by_id))
        .with_state(mc)
}
