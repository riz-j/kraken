use crate::{
    schemas::{city_schema::CitySummarizedSchema, country_schema::CountrySummarizedSchema},
    stores::city_store,
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

pub fn askama_router() -> Router {
    Router::new().route("/pages/cities/:city_id", get(render_city_page))
}
