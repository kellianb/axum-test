use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct LoginSession {
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub user_id: i32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct LoginUserData {
    pub id: i32,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateLogin {
    pub username: String,
    pub password: String,
}
