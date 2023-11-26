use axum::http::StatusCode;
use axum::routing::post;
use axum::{extract::Path, routing::get, Json, Router};
use kraken::clients::{city_client, country_client};
use kraken::models::country_model::{InsertCountry, SelectCountry};

async fn get_country_by_id(Path(country_id): Path<i64>) -> String {
    let country = country_client::select_country(country_id).await.unwrap();

    format!("Country: {:?}", country)
}

async fn create_country(Json(payload): Json<InsertCountry>) -> (StatusCode, String) {
    country_client::insert_country(&payload).await.unwrap();

    (StatusCode::CREATED, format!("Created"))
}

async fn get_city_by_id(Path(city_id): Path<i64>) -> String {
    let city = city_client::select_city(city_id).await.unwrap();
    let country = city.get_country().await.unwrap();

    format!("City: {:?}\nCountry: {:?}", city, country)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    kraken::db::init_pool().await;

    let app = Router::new()
        .route("/country/:country_id", get(get_country_by_id))
        .route("/country", post(create_country))
        .route("/city/:city_id", get(get_city_by_id));

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
