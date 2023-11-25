use mysqlx::clients::country_client;
// use mysqlx::models::country_model::InsertCountry;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_file = "./my_database.db";

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", db_file))
        .await?;

    // mysqlx::create_tables(&pool).await?;

    // country_client::insert_country(
    //     &pool,
    //     &InsertCountry {
    //         name: "Poland".to_string(),
    //     },
    // )
    // .await?;

    let some_country = country_client::select_country(&pool, 6).await?;

    println!("{:?}", some_country);

    Ok(())
}
