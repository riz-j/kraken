use crate::models::country_model::{InsertCountry, SelectCountry, UpdateCountry};
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

pub async fn update_country(
    pool: &Pool<Sqlite>,
    country_id: i64,
    update_country: &UpdateCountry,
) -> Result<(), Error> {
    sqlx::query(
        "
        UPDATE countries
        SET name = COALESCE($1, name)
        WHERE id = $2;
        ",
    )
    .bind(&update_country.name)
    .bind(country_id)
    .execute(pool)
    .await?;

    println!("Country updated!");

    Ok(())
}
