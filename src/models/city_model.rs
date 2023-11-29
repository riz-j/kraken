use crate::{
    clients::country_client,
    schemas::city_schema::{CityExtendedSchema, CitySummarizedSchema},
};
use serde::Serialize;
use sqlx::Error;

use super::country_model::SelectCountry;

#[derive(Debug, Serialize, sqlx::FromRow)]
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
    pub async fn get_country(&self) -> Result<SelectCountry, Error> {
        country_client::select_country(self.country_id.unwrap()).await
    }

    pub fn into_summarized_schema(&self) -> CitySummarizedSchema {
        CitySummarizedSchema {
            id: self.id,
            name: self.name.clone(),
        }
    }

    pub async fn into_extended_schema(&self) -> CityExtendedSchema {
        let country = self.get_country().await.unwrap();

        CityExtendedSchema {
            id: self.id,
            name: self.name.clone(),
            country: country,
        }
    }
}
