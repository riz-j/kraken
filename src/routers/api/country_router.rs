use crate::{
    ctx::Ctx,
    mc::ModelController,
    models::country_model::{CountryInsert, CountryUpdate},
    schemas::country_schema::{CountryExtendedSchema, CountrySummarizedSchema},
    stores::base::BaseStore,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    routing::get,
    Json, Router,
};

async fn list_countries(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Vec<CountrySummarizedSchema>>), Response> {
    let countries = mc.country_store.list(&ctx).await.unwrap();
    let countries_summ = countries
        .iter()
        .map(|country| country.into_summarized_schema())
        .collect();

    Ok((StatusCode::OK, Json(countries_summ)))
}

async fn get_country_by_id(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(country_id): Path<u32>,
) -> Json<CountryExtendedSchema> {
    let country = mc.country_store.select(&ctx, country_id).await.unwrap();

    Json(country.into_extended_schema(mc, &ctx).await)
}

async fn create_country(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(payload): Json<CountryInsert>,
) -> StatusCode {
    mc.country_store.insert(&ctx, payload).await.unwrap();

    StatusCode::CREATED
}

async fn update_country(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(country_id): Path<u32>,
    Json(payload): Json<CountryUpdate>,
) -> StatusCode {
    mc.country_store
        .update(&ctx, country_id, payload)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}

async fn delete_country(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(country_id): Path<u32>,
) -> StatusCode {
    mc.country_store
        .delete_country(&ctx, country_id)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}

pub fn country_router(mc: ModelController) -> Router {
    Router::new()
        .route("/countries", get(list_countries).post(create_country))
        .route(
            "/countries/:country_id",
            get(get_country_by_id)
                .patch(update_country)
                .delete(delete_country),
        )
        .with_state(mc)
}
