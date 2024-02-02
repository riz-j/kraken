use crate::stores::{city_store::CityStore, country_store::CountryStore};
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct ModelController {
    pub country_store: CountryStore,
    pub city_store: CityStore,
}

impl ModelController {
    pub fn new() -> Self {
        Self {
            country_store: CountryStore::new(),
            city_store: CityStore::new(),
        }
    }
}
