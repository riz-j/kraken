use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct InsertUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
