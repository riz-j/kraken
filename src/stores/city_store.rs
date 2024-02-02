use super::base::BaseStore;
use crate::ctx::Ctx;
use crate::db::{self, POOL};
use crate::models::city_model::{CityInsert, CitySelect, CityUpdate};
use async_trait::async_trait;
use sqlx::Error;

#[derive(Clone)]
pub struct CityStore;

impl CityStore {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BaseStore<CitySelect, CityInsert, CityUpdate> for CityStore {
    async fn list(&self, _ctx: &Ctx) -> Result<Vec<CitySelect>, sqlx::Error> {
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

    async fn select(&self, _ctx: &Ctx, id: u32) -> Result<CitySelect, sqlx::Error> {
        let city = sqlx::query_as::<_, CitySelect>(
            "
            SELECT id, name, country_id
            FROM cities WHERE id = $1;
            ",
        )
        .bind(id)
        .fetch_one(db::POOL.get().unwrap())
        .await?;

        Ok(city)
    }

    async fn insert(&self, _ctx: &Ctx, item: CityInsert) -> Result<u32, sqlx::Error> {
        let result = sqlx::query(
            "
            INSERT INTO cities (name, country_id)
            VALUES ($1, $2);
            ",
        )
        .bind(item.name.to_string())
        .bind(item.country_id)
        .execute(db::POOL.get().unwrap())
        .await?;

        Ok(result.last_insert_rowid() as u32)
    }
    async fn update(&self, _ctx: &Ctx, id: u32, item: CityUpdate) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            UPDATE cities
            SET 
                name = COALESCE($1, name),
                country_id = COALESCE($2, country_id)
            WHERE id = $3;
            ",
        )
        .bind(item.name)
        .bind(item.country_id)
        .bind(id)
        .execute(db::POOL.get().unwrap())
        .await?;

        Ok(())
    }
}

impl CityStore {
    pub async fn list_cities_by_country(
        &self,
        _ctx: &Ctx,
        id: u32,
    ) -> Result<Vec<CitySelect>, Error> {
        let cities = sqlx::query_as::<_, CitySelect>(
            "
            SELECT *
            FROM cities
            WHERE country_id = $1;
        ",
        )
        .bind(id)
        .fetch_all(POOL.get().unwrap())
        .await?;

        Ok(cities)
    }
}
