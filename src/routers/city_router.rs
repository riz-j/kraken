use axum::{extract::Path, routing::get, Json, Router};
use serde::ser::SerializeTupleStruct;

use crate::{
    clients::city_client,
    models::{city_model::SelectCity, country_model::SelectCountry},
};

async fn get_city_by_id(Path(city_id): Path<i64>) -> Json<(SelectCity, SelectCountry)> {
    let city = city_client::select_city(city_id).await.unwrap();
    let country = city.get_country().await.unwrap();

    Json((city, country))
}

async fn list_cities() -> Json<Vec<SelectCity>> {
    let cities = city_client::list_cities().await.unwrap();

    Json(cities)
}

pub fn city_router() -> Router {
    Router::new()
        .route("/cities/:city_id", get(get_city_by_id))
        .route("/cities", get(list_cities))
}
