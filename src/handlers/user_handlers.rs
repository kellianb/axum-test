use crate::models::user_models::*;
use chrono::Utc;
use sqlx::{query, query_as, Error, Pool, Postgres};

pub async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<User>, Error> {
    query_as!(User, "SELECT id, username, role, created_at FROM users")
        .fetch_all(pool)
        .await
}

pub async fn get(id: i32, pool: &Pool<Postgres>) -> Result<User, Error> {
    query_as!(
        User,
        "SELECT id, username, role, created_at FROM users WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create(user_data: CreateUser, pool: &Pool<Postgres>) -> Result<NewUser, Error> {
    query_as!(
        NewUser,
        r#"INSERT INTO users (id, username, role, created_at) 
        VALUES (DEFAULT, $1, $2, $3)
        RETURNING id"#,
        user_data.username,
        user_data.role,
        Utc::now().naive_utc(),
    )
    .fetch_one(pool)
    .await
}

pub async fn delete(id: i32, pool: &Pool<Postgres>) -> Result<u64, Error> {
    Ok(query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected())
}
