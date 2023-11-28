use serde::Serialize;

use crate::models::city_model::SelectCity;

#[derive(Debug, Serialize)]
pub struct CountryExtendedSchema {
    pub id: i64,
    pub name: String,
    pub continent: Option<String>,
    pub cities: Vec<SelectCity>,
}
