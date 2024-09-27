use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub role_id: i32,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct NewUser {
    pub id: i32,
}
