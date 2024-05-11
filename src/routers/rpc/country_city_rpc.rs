use crate::ctx::Ctx;
use crate::mc::ModelController;
use crate::schemas::{
    city_schema::{CityExtendedSchema, CitySummarizedSchema},
    country_schema::{CountryExtendedSchema, CountrySummarizedSchema},
};
use crate::stores::base::BaseStore;
use serde::Deserialize;

use super::rpc_router::EmptyParams;

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCityParams {
    id: u32,
}
pub async fn get_city(mc: ModelController, ctx: Ctx, params: GetCityParams) -> CityExtendedSchema {
    let city = mc.city_store.select(&ctx, params.id).await.unwrap();
    let city_extended = city.into_extended_schema(mc, &ctx).await;

    city_extended
}

pub async fn list_countries(
    mc: ModelController,
    ctx: Ctx,
    _: EmptyParams,
) -> Vec<CountrySummarizedSchema> {
    let countries = mc.country_store.list(&ctx).await.unwrap();
    let countries_summarized = countries
        .iter()
        .map(|country| country.into_summarized_schema())
        .collect();

    countries_summarized
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCountryParams {
    id: u32,
}
pub async fn get_country(
    mc: ModelController,
    ctx: Ctx,
    params: GetCountryParams,
) -> CountryExtendedSchema {
    let country = mc.country_store.select(&ctx, params.id).await.unwrap();

    let country_extended = country.into_extended_schema(mc, &ctx).await;

    country_extended
}
