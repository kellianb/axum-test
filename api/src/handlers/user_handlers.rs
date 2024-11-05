use chrono::Utc;
use models::user_models::*;
use sqlx::{query, query_as, Pool, Postgres};

pub async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    query_as!(
        User,
        "SELECT users.id, users.username, r.name AS role, users.created_at FROM users
                                      JOIN public.roles r on r.id = users.role_id
                                      WHERE deleted = false"
    )
    .fetch_all(pool)
    .await
}

pub async fn get(id: i32, pool: &Pool<Postgres>) -> Result<User, sqlx::Error> {
    query_as!(
        User,
        "SELECT users.id, users.username, r.name AS role, users.created_at FROM users
                                      JOIN public.roles r on r.id = users.role_id
                                      WHERE users.id = $1 AND deleted = false",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create(user_data: CreateUser, pool: &Pool<Postgres>) -> Result<NewUser, sqlx::Error> {
    query_as!(
        NewUser,
        r#"INSERT INTO users (id, username, password_hash, role_id, created_at, deleted)
        VALUES (DEFAULT, $1, $2, $3, $4, DEFAULT)
        RETURNING id"#,
        user_data.username,
        user_data.password,
        user_data.role_id,
        Utc::now().naive_utc(),
    )
    .fetch_one(pool)
    .await
}

pub async fn delete(id: i32, pool: &Pool<Postgres>) -> Result<u64, sqlx::Error> {
    Ok(query!("UPDATE users SET deleted = TRUE WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected())
}
