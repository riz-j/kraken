use serde::Serialize;
use ts_rs::TS;

use crate::models::city_model::SelectCity;

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CountryExtendedSchema {
    pub id: i64,
    pub name: String,
    pub continent: Option<String>,
    pub cities: Vec<SelectCity>,
    pub is_archived: bool,
}
