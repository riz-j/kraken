use super::city_model::SelectCity;
use crate::clients::city_client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct SelectCountry {
    pub id: i64,
    pub name: String,
    pub continent: Option<String>,
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
}

impl SelectCountry {
    pub async fn get_cities(&self) -> Vec<SelectCity> {
        city_client::list_cities_by_country(self.id).await.unwrap()
    }
}
