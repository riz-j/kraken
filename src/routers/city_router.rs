use crate::{
    ctx::Ctx,
    mc::ModelController,
    schemas::city_schema::{CityExtendedSchema, CitySummarizedSchema},
    stores::base::BaseStore,
};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

async fn get_city_by_id(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(city_id): Path<u32>,
) -> Json<CityExtendedSchema> {
    let city = mc.city_store.select(&ctx, city_id).await.unwrap();

    Json(city.into_extended_schema(mc, &ctx).await)
}

async fn list_cities(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Json<Vec<CitySummarizedSchema>> {
    let cities = mc.city_store.list(&ctx).await.unwrap();
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
