use serde::Serialize;
use ts_rs::TS;

use super::country_schema::CountrySummarizedSchema;

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CitySummarizedSchema {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CityExtendedSchema {
    pub id: u32,
    pub name: String,
    pub country: CountrySummarizedSchema,
}
