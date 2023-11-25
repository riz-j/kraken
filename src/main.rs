use mysqlx::{
    clients::country_client,
    models::country_model::{InsertCountry, UpdateCountry},
};
// use mysqlx::models::country_model::InsertCountry;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_file = "./my_database.db";

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", db_file))
        .await?;

    // country_client::insert_country(
    //     &pool,
    //     &InsertCountry {
    //         name: "Angola".to_string(),
    //         continent: None,
    //     },
    // )
    // .await?;

    let some_country = country_client::select_country(&pool, 7).await?;
    println!("{:?}", some_country);

    // let update_country = UpdateCountry {
    //     name: None,
    //     continent: None,
    // };
    // mysqlx::clients::country_client::update_country(&pool, 2, &update_country).await?;

    Ok(())
}
