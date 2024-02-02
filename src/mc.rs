use crate::stores::{city_store::CityStore, country_store::CountryStore, user_store::UserStore};
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct ModelController {
    pub country_store: CountryStore,
    pub city_store: CityStore,
    pub user_store: UserStore,
}

impl ModelController {
    pub fn new() -> Self {
        Self {
            country_store: CountryStore::new(),
            city_store: CityStore::new(),
            user_store: UserStore::new(),
        }
    }
}
