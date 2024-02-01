use super::base::BaseStore;
use crate::ctx::Ctx;
use crate::db;
use crate::models::country_model::{CountryInsert, CountrySelect, CountryUpdate};
use async_trait::async_trait;
use sqlx::Error;

#[derive(Clone)]
pub struct CountryStore;

impl CountryStore {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BaseStore<CountrySelect, CountryInsert, CountryUpdate> for CountryStore {
    async fn list(&self, _ctx: &Ctx) -> Result<Vec<CountrySelect>, sqlx::Error> {
        let countries = sqlx::query_as::<_, CountrySelect>(
            "
            SELECT *
            FROM countries;
            ",
        )
        .fetch_all(db::POOL.get().unwrap())
        .await?;

        Ok(countries)
    }

    async fn select(&self, _ctx: &Ctx, id: i64) -> Result<CountrySelect, sqlx::Error> {
        let country = sqlx::query_as::<_, CountrySelect>(
            "
            SELECT *
            FROM countries WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_one(db::POOL.get().unwrap())
        .await?;

        Ok(country)
    }

    async fn insert(&self, ctx: &Ctx, item: CountryInsert) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO countries (name, continent)
            VALUES ($1, $2);
            ",
        )
        .bind(item.name.to_string())
        .bind(&item.continent)
        .execute(db::POOL.get().unwrap())
        .await?;

        println!("Country '{}' has been inserted!", item.name.to_string());

        Ok(())
    }

    async fn update(&self, ctx: &Ctx, id: i64, item: CountryUpdate) -> Result<(), sqlx::Error> {
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
        .bind(&item.name)
        .bind(&item.continent)
        .bind(&item.is_archived)
        .bind(id)
        .execute(db::POOL.get().unwrap())
        .await?;

        println!("Country updated!");

        Ok(())
    }
}

impl CountryStore {
    pub async fn delete_country(&self, country_id: i64) -> Result<(), Error> {
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
}
