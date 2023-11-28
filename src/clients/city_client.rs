use crate::db;
use crate::models::city_model::{InsertCity, SelectCity};
use sqlx::Error;

pub async fn insert_city(insert_city: &InsertCity) -> Result<(), Error> {
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

pub async fn select_city(city_id: i64) -> Result<SelectCity, Error> {
    let city = sqlx::query_as::<_, SelectCity>(
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

pub async fn list_cities() -> Result<Vec<SelectCity>, Error> {
    let cities = sqlx::query_as::<_, SelectCity>(
        "
        SELECT * 
        FROM cities;
    ",
    )
    .fetch_all(db::POOL.get().unwrap())
    .await?;

    Ok(cities)
}
