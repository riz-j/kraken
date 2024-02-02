use crate::db::{self, POOL};
use crate::models::city_model::{CityInsert, CitySelect};
use sqlx::Error;

pub async fn insert_city(insert_city: &CityInsert) -> Result<(), Error> {
    sqlx::query(
        "
        INSERT INTO cities (name, country_id)
        VALUES ($1, $2);
    ",
    )
    .bind(insert_city.name.to_string())
    .bind(insert_city.country_id)
    .execute(db::POOL.get().unwrap())
    .await?;

    println!("City '{}' has been inserted!", insert_city.name);

    Ok(())
}

pub async fn select_city(city_id: u32) -> Result<CitySelect, Error> {
    let city = sqlx::query_as::<_, CitySelect>(
        "
        SELECT id, name, country_id
        FROM cities WHERE id = $1;
        ",
    )
    .bind(city_id)
    .fetch_one(db::POOL.get().unwrap())
    .await?;

    Ok(city)
}

pub async fn list_cities() -> Result<Vec<CitySelect>, Error> {
    let cities = sqlx::query_as::<_, CitySelect>(
        "
        SELECT * 
        FROM cities;
    ",
    )
    .fetch_all(db::POOL.get().unwrap())
    .await?;

    Ok(cities)
}

pub async fn list_cities_by_country(country_id: u32) -> Result<Vec<CitySelect>, Error> {
    let cities = sqlx::query_as::<_, CitySelect>(
        "
        SELECT *
        FROM cities
        WHERE country_id = $1;
    ",
    )
    .bind(country_id)
    .fetch_all(POOL.get().unwrap())
    .await?;

    Ok(cities)
}
