use models::login_models::*;
use sqlx::{query, query_as, Pool, Postgres};

// Get a users password hash and id by the username
pub async fn get_login_data(
    username: &String,
    pool: &Pool<Postgres>,
) -> Result<LoginUserData, sqlx::Error> {
    query_as!(
        LoginUserData,
        "SELECT users.id, users.password_hash
            FROM users
            WHERE username = $1;",
        username
    )
    .fetch_one(pool)
    .await
}

pub async fn get_session(
    token: &String,
    pool: &Pool<Postgres>,
) -> Result<LoginSession, sqlx::Error> {
    query_as!(
        LoginSession,
        "SELECT *
                FROM sessions
                WHERE token = $1;",
        token,
    )
    .fetch_one(pool)
    .await
}

pub async fn create(
    login_session_data: &LoginSession,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    match query!(
        "
        INSERT INTO sessions (token, user_id, created_at, expires_at)
        VALUES ($1, $2, $3, $4)",
        login_session_data.token,
        login_session_data.user_id,
        login_session_data.created_at,
        login_session_data.expires_at,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
