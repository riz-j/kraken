use crate::{
    ctx::Ctx,
    mc::ModelController,
    models::city_model::CityInsert,
    schemas::city_schema::{CityExtendedSchema, CitySummarizedSchema},
    stores::base::BaseStore,
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Form, Json, Router,
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

async fn create_city(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Form(payload): Form<CityInsert>,
) -> axum::http::Response<Body> {
    let city_id = match mc.city_store.insert(&ctx, payload).await {
        Ok(value) => value,
        Err(_) => {
            return axum::response::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    };
    let location = format!("/pages/cities/{}", city_id);

    axum::response::Response::builder()
        .status(StatusCode::CREATED)
        .header("Location", &location)
        .header("HX-Refresh", "true")
        .body(Body::empty())
        .unwrap()
}

pub fn city_router(mc: ModelController) -> Router {
    Router::new()
        .route("/cities", get(list_cities).post(create_city))
        .route("/cities/:city_id", get(get_city_by_id))
        .with_state(mc)
}
