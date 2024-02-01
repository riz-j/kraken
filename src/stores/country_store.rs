use crate::ctx::Ctx;
use crate::db;
use crate::models::country_model::{CountryInsert, CountrySelect, CountryUpdate};
use async_trait::async_trait;
use sqlx::Error;

use super::base::BaseStore;

#[derive(Clone)]
pub struct CountryStore;

impl CountryStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn insert_country(&self, country: &CountryInsert) -> Result<(), Error> {
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

    pub async fn update_country(
        &self,
        country_id: i64,
        update_country: &CountryUpdate,
    ) -> Result<(), Error> {
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

#[async_trait]
impl BaseStore<CountrySelect> for CountryStore {
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

    async fn get(&self, _ctx: &Ctx, id: i64) -> Result<CountrySelect, sqlx::Error> {
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
}
