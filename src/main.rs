use axum::http::StatusCode;
use axum::routing::{get_service, post};
use axum::{extract::Path, routing::get, Json, Router};
use kraken::clients::{city_client, country_client};
use kraken::models::city_model::SelectCity;
use kraken::models::country_model::{InsertCountry, SelectCountry};
use tower_http::services::ServeDir;

async fn get_country_by_id(Path(country_id): Path<i64>) -> Json<SelectCountry> {
    let country = country_client::select_country(country_id).await.unwrap();

    Json(country)
}

async fn list_countries() -> Json<Vec<SelectCountry>> {
    let countries = country_client::list_countries().await.unwrap();

    Json(countries)
}

async fn create_country(Json(payload): Json<InsertCountry>) -> StatusCode {
    country_client::insert_country(&payload).await.unwrap();

    StatusCode::CREATED
}

async fn get_city_by_id(Path(city_id): Path<i64>) -> Json<(SelectCity, SelectCountry)> {
    let city = city_client::select_city(city_id).await.unwrap();
    let country = city.get_country().await.unwrap();

    Json((city, country))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    kraken::db::init_pool().await;

    let app = Router::new()
        .route("/countries", get(list_countries))
        .route("/countries/:country_id", get(get_country_by_id))
        .route("/countries", post(create_country))
        .route("/cities/:city_id", get(get_city_by_id))
        .fallback_service(routes_static());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    let some_city = city_client::select_city(2).await?;
    let some_country = some_city.get_country().await?;

    println!("{:?}", some_city);
    println!("{:?}", some_country);

    Ok(())
}
