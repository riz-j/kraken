use crate::models::country_model::{InsertCountry, SelectCountry, UpdateCountry};
use sqlx::{Error, Pool, Sqlite};

pub async fn insert_country(pool: &Pool<Sqlite>, country: &InsertCountry) -> Result<(), Error> {
    sqlx::query(
        "
        INSERT INTO countries (name, continent)
        VALUES ($1, $2);
    ",
    )
    .bind(country.name.to_string())
    .bind(country.continent.to_string())
    .execute(pool)
    .await?;

    println!("Country '{}' has been inserted!", country.name.to_string());

    Ok(())
}

pub async fn select_country(pool: &Pool<Sqlite>, country_id: i64) -> Result<SelectCountry, Error> {
    let country = sqlx::query_as::<_, SelectCountry>(
        "
        SELECT id, name, continent 
        FROM countries WHERE id = $1
        ",
    )
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
        SET 
            name = COALESCE($1, name),
            continent = COALESCE($2, continent)
        WHERE id = $3;
        ",
    )
    .bind(&update_country.name)
    .bind(&update_country.continent)
    .bind(country_id)
    .execute(pool)
    .await?;

    println!("Country updated!");

    Ok(())
}
