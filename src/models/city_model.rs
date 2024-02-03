use crate::{
    ctx::Ctx,
    mc::ModelController,
    schemas::city_schema::{CityExtendedSchema, CitySummarizedSchema},
    stores::base::BaseStore,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::country_model::CountrySelect;

#[derive(Debug, Serialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct CitySelect {
    pub id: u32,
    pub name: String,
    pub country_id: Option<u32>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, TS)]
#[ts(export)]
#[allow(dead_code)]
pub struct CityInsert {
    pub name: String,
    pub country_id: Option<u32>,
}

#[derive(Deserialize, TS)]
#[ts(export)]
#[allow(dead_code)]
pub struct CityUpdate {
    pub name: Option<String>,
    pub country_id: Option<u32>,
}

impl CitySelect {
    pub async fn get_country(
        &self,
        mc: ModelController,
        ctx: &Ctx,
    ) -> Result<CountrySelect, sqlx::Error> {
        mc.country_store.select(ctx, self.country_id.unwrap()).await
    }

    pub fn into_summarized_schema(&self) -> CitySummarizedSchema {
        CitySummarizedSchema {
            id: self.id,
            name: self.name.clone(),
        }
    }

    pub async fn into_extended_schema(&self, mc: ModelController, ctx: &Ctx) -> CityExtendedSchema {
        let country = self.get_country(mc, ctx).await.unwrap();
        let country_summarized_schema = country.into_summarized_schema();

        CityExtendedSchema {
            id: self.id,
            name: self.name.clone(),
            country: country_summarized_schema,
        }
    }
}
