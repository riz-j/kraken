use axum::{extract::Path, routing::get, Error, Router};
use mysqlx::{
    clients::{city_client, country_client},
    models::{
        city_model::{InsertCity, SelectCity},
        country_model::{InsertCountry, SelectCountry, UpdateCountry},
    },
};
// use mysqlx::models::country_model::InsertCountry;
use sqlx::sqlite::SqlitePoolOptions;

async fn get_country_by_id(Path(user_id): Path<i64>) -> String {
    let db_file = "./my_database.db";
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", db_file))
        .await
        .unwrap();

    let country = country_client::select_country(&pool, user_id)
        .await
        .unwrap();

    format!("Country: {:?}", country)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let app = Router::new().route("/country/:country_id", get(get_country_by_id));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    let db_file = "./my_database.db";
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", db_file))
        .await?;

    let some_city = city_client::select_city(&pool, 2).await?;
    let some_country = some_city.get_country(&pool).await?;

    println!("{:?}", some_city);
    println!("{:?}", some_country);

    Ok(())
}
