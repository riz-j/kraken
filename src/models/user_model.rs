use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserId {
    pub id: i64,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct SelectUser {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct InsertUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
