use serde::Serialize;
use ts_rs::TS;

use crate::models::country_model::SelectCountry;

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CitySummarizedSchema {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CityExtendedSchema {
    pub id: i64,
    pub name: String,
    pub country: SelectCountry,
}
