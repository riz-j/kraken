use serde::Serialize;
use ts_rs::TS;

use super::city_schema::CitySummarizedSchema;

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CountrySummarizedSchema {
    pub id: u32,
    pub name: String,
    pub continent: Option<String>,
    pub is_archived: bool,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CountryExtendedSchema {
    pub id: u32,
    pub name: String,
    pub continent: Option<String>,
    pub cities: Vec<CitySummarizedSchema>,
    pub is_archived: bool,
}
