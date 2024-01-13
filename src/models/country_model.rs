use super::city_model::CitySelect;
use crate::{
    schemas::country_schema::{CountryExtendedSchema, CountrySummarizedSchema},
    stores::city_store,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Debug, sqlx::FromRow, TS)]
pub struct CountrySelect {
    pub id: i64,
    pub name: String,
    pub continent: Option<String>,
    pub is_archived: bool,
}

#[derive(Deserialize, TS)]
#[ts(export)]
#[allow(dead_code)]
pub struct CountryInsert {
    pub name: String,
    pub continent: String,
}

#[derive(Deserialize, TS)]
#[ts(export)]
#[allow(dead_code)]
pub struct CountryUpdate {
    pub name: Option<String>,
    pub continent: Option<String>,
    pub is_archived: Option<bool>,
}

impl CountrySelect {
    pub async fn get_cities(&self) -> Vec<CitySelect> {
        city_store::list_cities_by_country(self.id).await.unwrap()
    }

    pub fn into_summarized_schema(&self) -> CountrySummarizedSchema {
        CountrySummarizedSchema {
            id: self.id,
            name: self.name.clone(),
            continent: self.continent.clone(),
            is_archived: self.is_archived,
        }
    }

    pub async fn into_extended_schema(&self) -> CountryExtendedSchema {
        let cities = self.get_cities().await;
        let cities_summarized = cities
            .iter()
            .map(|city| city.into_summarized_schema())
            .collect();

        CountryExtendedSchema {
            id: self.id,
            name: self.name.clone(),
            continent: self.continent.clone(),
            cities: cities_summarized,
            is_archived: self.is_archived,
        }
    }
}
