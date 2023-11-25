use crate::models::city_model::InsertCity;
use sqlx::{Error, Pool, Sqlite};

pub async fn insert_city(pool: &Pool<Sqlite>, insert_city: &InsertCity) -> Result<(), Error> {
    sqlx::query(
        "
        INSERT INTO cities (name, country_id)
        VALUES ($1, $2);
    ",
    )
    .bind(insert_city.name.to_string())
    .bind(insert_city.country_id)
    .execute(pool)
    .await?;

    println!("City '{}' has been inserted!", insert_city.name);

    Ok(())
}
