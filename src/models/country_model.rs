use super::city_model::SelectCity;
use crate::{
    schemas::country_schema::{CountryExtendedSchema, CountrySummarizedSchema},
    stores::city_store,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Debug, sqlx::FromRow, TS)]
pub struct SelectCountry {
    pub id: i64,
    pub name: String,
    pub continent: Option<String>,
    pub is_archived: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct InsertCountry {
    pub name: String,
    pub continent: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct UpdateCountry {
    pub name: Option<String>,
    pub continent: Option<String>,
    pub is_archived: Option<bool>,
}

impl SelectCountry {
    pub async fn get_cities(&self) -> Vec<SelectCity> {
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

        CountryExtendedSchema {
            id: self.id,
            name: self.name.clone(),
            continent: self.continent.clone(),
            cities: cities,
            is_archived: self.is_archived,
        }
    }
}
