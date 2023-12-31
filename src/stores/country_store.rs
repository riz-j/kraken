use crate::db;
use crate::models::country_model::{InsertCountry, SelectCountry, UpdateCountry};
use sqlx::Error;

pub async fn insert_country(country: &InsertCountry) -> Result<(), Error> {
    sqlx::query(
        "
        INSERT INTO countries (name, continent)
        VALUES ($1, $2);
    ",
    )
    .bind(country.name.to_string())
    .bind(&country.continent)
    .execute(db::POOL.get().unwrap())
    .await?;

    println!("Country '{}' has been inserted!", country.name.to_string());

    Ok(())
}

pub async fn list_countries() -> Result<Vec<SelectCountry>, Error> {
    let countries = sqlx::query_as::<_, SelectCountry>(
        "
        SELECT *
        FROM countries;
        ",
    )
    .fetch_all(db::POOL.get().unwrap())
    .await?;

    Ok(countries)
}

pub async fn select_country(country_id: i64) -> Result<SelectCountry, Error> {
    let country = sqlx::query_as::<_, SelectCountry>(
        "
        SELECT *
        FROM countries WHERE id = $1
        ",
    )
    .bind(country_id)
    .fetch_one(db::POOL.get().unwrap())
    .await?;

    Ok(country)
}

pub async fn update_country(country_id: i64, update_country: &UpdateCountry) -> Result<(), Error> {
    sqlx::query(
        "
        UPDATE countries
        SET 
            name = COALESCE($1, name),
            continent = COALESCE($2, continent),
            is_archived = COALESCE($3, is_archived)
        WHERE id = $4;
        ",
    )
    .bind(&update_country.name)
    .bind(&update_country.continent)
    .bind(&update_country.is_archived)
    .bind(country_id)
    .execute(db::POOL.get().unwrap())
    .await?;

    println!("Country updated!");

    Ok(())
}

pub async fn delete_country(country_id: i64) -> Result<(), Error> {
    sqlx::query(
        "
        DELETE FROM countries
        WHERE id = $1;
        ",
    )
    .bind(country_id)
    .execute(db::POOL.get().unwrap())
    .await?;

    Ok(())
}
