use serde::Serialize;

use crate::models::country_model::SelectCountry;

#[derive(Debug, Serialize)]
pub struct CitySummarizedSchema {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CityExtendedSchema {
    pub id: i64,
    pub name: String,
    pub country: SelectCountry,
}
