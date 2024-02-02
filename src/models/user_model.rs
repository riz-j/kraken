use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserId {
    pub id: u32,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserSelect {
    pub id: u32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct UserInsert {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
