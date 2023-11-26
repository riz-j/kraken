use crate::clients::country_client;
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
}
