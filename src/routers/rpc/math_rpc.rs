use crate::{
    ctx::Ctx, mc::ModelController, schemas::city_schema::CitySummarizedSchema,
    stores::base::BaseStore,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmptyParams {}

#[derive(Deserialize)]
pub struct ParamsForAdd {
    pub a: f32,
    pub b: f32,
}
pub async fn add(_mc: ModelController, _ctx: Ctx, params: ParamsForAdd) -> f32 {
    params.a + params.b
}

#[derive(Deserialize)]
pub struct ParamsForSubtract {
    pub a: f32,
    pub b: f32,
}
pub async fn subtract(_mc: ModelController, _ctx: Ctx, params: ParamsForSubtract) -> f32 {
    params.a - params.b
}

#[derive(Deserialize)]
pub struct ParamsForMultiply {
    pub a: f32,
    pub b: f32,
}
pub async fn multiply(_mc: ModelController, _ctx: Ctx, params: ParamsForMultiply) -> f32 {
    params.a * params.b
}

#[derive(Deserialize)]
pub struct ParamsForDivide {
    pub a: f32,
    pub b: f32,
}
pub async fn divide(_mc: ModelController, _ctx: Ctx, params: ParamsForDivide) -> f32 {
    params.a / params.b
}

pub async fn list_cities(
    mc: ModelController,
    ctx: Ctx,
    _: EmptyParams,
) -> Vec<CitySummarizedSchema> {
    let cities = mc.city_store.list(&ctx).await.unwrap();
    let cities_summarized = cities
        .iter()
        .map(|city| city.into_summarized_schema())
        .collect();

    cities_summarized
}
