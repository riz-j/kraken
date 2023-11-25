mod models;
// use mysqlx::models::InsertCountry;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_file = "./my_database.db";

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", db_file))
        .await?;

    // mysqlx::create_tables(&pool).await?;

    // mysqlx::insert_country(
    //     &pool,
    //     &InsertCountry {
    //         name: "Poland".to_string(),
    //     },
    // )
    // .await?;

    let some_country = mysqlx::select_country(&pool, 6).await?;

    println!("{:?}", some_country);

    Ok(())
}
