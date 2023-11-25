use crate::models::country_model::{InsertCountry, SelectCountry};
use sqlx::{Error, Pool, Sqlite};

pub async fn insert_country(pool: &Pool<Sqlite>, country: &InsertCountry) -> Result<(), Error> {
    sqlx::query(
        "
        INSERT INTO countries (name)
        VALUES ($1);
    ",
    )
    .bind(country.name.to_string())
    .execute(pool)
    .await?;

    println!("Country '{}' has been inserted!", country.name.to_string());

    Ok(())
}

pub async fn select_country(pool: &Pool<Sqlite>, country_id: i64) -> Result<SelectCountry, Error> {
    let country =
        sqlx::query_as::<_, SelectCountry>("SELECT id, name FROM countries WHERE id = $1")
            .bind(country_id)
            .fetch_one(pool)
            .await?;

    Ok(country)
}
