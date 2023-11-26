use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct SelectCountry {
    pub id: i64,
    pub name: String,
    pub continent: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct InsertCountry {
    pub name: String,
    pub continent: String,
}

#[allow(dead_code)]
pub struct UpdateCountry {
    pub name: Option<String>,
    pub continent: Option<String>,
}
