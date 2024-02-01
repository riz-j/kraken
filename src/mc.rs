use crate::stores::country_store::CountryStore;
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct ModelController {
    pub country_store: CountryStore,
}

impl ModelController {
    pub fn new() -> Self {
        Self {
            country_store: CountryStore::new(),
        }
    }
}
