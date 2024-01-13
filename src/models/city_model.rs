use crate::{
    schemas::city_schema::{CityExtendedSchema, CitySummarizedSchema},
    stores::country_store,
};
use serde::Serialize;
use sqlx::Error;
use ts_rs::TS;

use super::country_model::CountrySelect;

#[derive(Debug, Serialize, sqlx::FromRow, TS)]
#[allow(dead_code)]
pub struct SelectCity {
    pub id: i64,
    pub name: String,
    pub country_id: Option<i64>,
}

#[allow(dead_code)]
pub struct InsertCity {
    pub name: String,
    pub country_id: Option<i64>,
}

impl SelectCity {
    pub async fn get_country(&self) -> Result<CountrySelect, Error> {
        country_store::select_country(self.country_id.unwrap()).await
    }

    pub fn into_summarized_schema(&self) -> CitySummarizedSchema {
        CitySummarizedSchema {
            id: self.id,
            name: self.name.clone(),
        }
    }

    pub async fn into_extended_schema(&self) -> CityExtendedSchema {
        let country = self.get_country().await.unwrap();
        let country_summarized_schema = country.into_summarized_schema();

        CityExtendedSchema {
            id: self.id,
            name: self.name.clone(),
            country: country_summarized_schema,
        }
    }
}
