use crate::{
    schemas::{
        city_schema::CitySummarizedSchema,
        country_schema::{CountryExtendedSchema, CountrySummarizedSchema},
    },
    stores::{city_store, country_store},
};
use askama::Template;
use axum::{extract::Path, response::Html, routing::get, Router};

#[derive(Template)]
#[template(path = "city.template.html")]
#[allow(dead_code)]
struct CityTemplate {
    city: CitySummarizedSchema,
    country: CountrySummarizedSchema,
}

async fn render_city_page(Path(city_id): Path<i64>) -> Html<String> {
    let city = city_store::select_city(city_id).await.unwrap();
    let country = city.get_country().await.unwrap();

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

async fn render_country_page(Path(country_id): Path<i64>) -> Html<String> {
    let country = country_store::select_country(country_id).await.unwrap();
    let country_extended = country.into_extended_schema().await;

    let country_template = CountryTemplate {
        country: country_extended,
    };

    let template_string = country_template.render().unwrap();
    Html(template_string)
}

pub fn askama_router() -> Router {
    Router::new()
        .route("/pages/cities/:city_id", get(render_city_page))
        .route("/pages/countries/:country_id", get(render_country_page))
}
