use mysqlx::{
    clients::{city_client, country_client},
    models::{
        city_model::{InsertCity, SelectCity},
        country_model::{InsertCountry, SelectCountry, UpdateCountry},
    },
};
// use mysqlx::models::country_model::InsertCountry;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
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
