use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::Utc;
use models::user_models::*;
use sqlx::{query, query_as, Pool, Postgres};
use std::fmt;

// Define a custom error type
#[derive(Debug)]
pub enum Error {
    Argon2Error(argon2::password_hash::Error),
    DatabaseError(sqlx::Error),
    // You can add more error types here
}

// Implement the Display trait for your custom error type
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Argon2Error(e) => write!(f, "Argon2 error: {}", e),
            Error::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

// Convert Argon2 errors to MyError
impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Error::Argon2Error(err)
    }
}

// Convert Sqlx errors to MyError
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::DatabaseError(err)
    }
}

impl std::error::Error for Error {}

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

pub async fn create(user_data: CreateUser, pool: &Pool<Postgres>) -> Result<NewUser, Error> {
    // Generate SaltString
    let salt = SaltString::generate(&mut OsRng);
    println!("{salt}");

    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(user_data.password.as_bytes(), &salt)
        .map_err(|e| Error::from(e))?
        .to_string();

    println!("{password_hash}");

    query_as!(
        NewUser,
        r#"INSERT INTO users (id, username, password_hash, role_id, created_at, deleted)
        VALUES (DEFAULT, $1, $2, $3, $4, DEFAULT)
        RETURNING id"#,
        user_data.username,
        password_hash,
        user_data.role_id,
        Utc::now().naive_utc(),
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from(e))
}

pub async fn delete(id: i32, pool: &Pool<Postgres>) -> Result<u64, sqlx::Error> {
    Ok(query!("UPDATE users SET deleted = TRUE WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected())
}
