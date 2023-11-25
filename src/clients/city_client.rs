use crate::models::city_model::{InsertCity, SelectCity};
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

pub async fn select_city(pool: &Pool<Sqlite>, city_id: i64) -> Result<SelectCity, Error> {
    let city = sqlx::query_as::<_, SelectCity>(
        "
        SELECT id, name, country_id
        FROM cities WHERE id = $1;
        ",
    )
    .bind(city_id)
    .fetch_one(pool)
    .await?;

    Ok(city)
}
