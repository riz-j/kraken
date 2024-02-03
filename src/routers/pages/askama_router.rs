use crate::{
    ctx::Ctx,
    mc::ModelController,
    schemas::{
        city_schema::CitySummarizedSchema,
        country_schema::{CountryExtendedSchema, CountrySummarizedSchema},
    },
    stores::base::BaseStore,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "city.template.html")]
#[allow(dead_code)]
struct CityTemplate {
    city: CitySummarizedSchema,
    country: CountrySummarizedSchema,
}

async fn render_city_page(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(city_id): Path<u32>,
) -> Html<String> {
    let city = mc.city_store.select(&ctx, city_id).await.unwrap();
    let country = city.get_country(mc, &ctx).await.unwrap();

    let city_summarized = city.into_summarized_schema();
    let country_summarized = country.into_summarized_schema();

    let template = CityTemplate {
        city: city_summarized,
        country: country_summarized,
    };

    let template_string = template.render().unwrap();
    Html(template_string)
}

#[derive(Template)]
#[template(path = "country.template.html")]
#[allow(dead_code)]
struct CountryTemplate {
    country: CountryExtendedSchema,
}

async fn render_country_page(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(country_id): Path<u32>,
) -> Html<String> {
    let country = mc.country_store.select(&ctx, country_id).await.unwrap();
    let country_extended = country.into_extended_schema(mc, &ctx).await;

    let country_template = CountryTemplate {
        country: country_extended,
    };

    let template_string = country_template.render().unwrap();
    Html(template_string)
}

pub fn askama_router(mc: ModelController) -> Router {
    Router::new()
        .route("/cities/:city_id", get(render_city_page))
        .route("/countries/:country_id", get(render_country_page))
        .with_state(mc)
}
