use crate::error::AppError;
use crate::handlers::user_handlers;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use models::user_models::*;
use serde_json::json;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub fn get_routes() -> Router {
    axum::Router::new()
        .route("/", axum::routing::get(get_all))
        .route("/:id", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::delete(delete))
}

pub async fn get_all(Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>) -> impl IntoResponse {
    let response = user_handlers::get_all(&pool).await;

    match response {
        Ok(val) => (StatusCode::OK, Json(&val)).into_response(),
        Err(_) => AppError::InternalServerError.into_response(),
    }
}

pub async fn get(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    let response = user_handlers::get(id, &pool).await;

    match response {
        Ok(val) => (StatusCode::OK, Json(&val)).into_response(),
        Err(sqlx::Error::RowNotFound) => AppError::UserNotFound.into_response(),
        Err(_) => AppError::InternalServerError.into_response(),
    }
}

pub async fn create(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
    Json(user_data): Json<CreateUser>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    // Replace password in user_data with hashed version
    let user_data = match argon2.hash_password(user_data.password.as_bytes(), &salt) {
        Ok(password_hash) => CreateUser {
            password: password_hash.to_string(),
            ..user_data
        },
        Err(_) => {
            return AppError::InvalidUserPassword.into_response();
        }
    };

    let response = user_handlers::create(user_data, &pool).await;

    match response {
        Ok(val) => (StatusCode::OK, Json(&val)).into_response(),
        Err(e) => match e {
            sqlx::Error::Database(_) => AppError::InvalidUserRole.into_response(),
            _ => AppError::InternalServerError.into_response(),
        },
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    let response = user_handlers::delete(id, &pool).await;

    match response {
        Ok(val) if val > 0 => {
            (StatusCode::OK, Json(json!({ "message" : "User deleted"}))).into_response()
        }
        Ok(_) => AppError::UserNotFound.into_response(),
        Err(_) => AppError::InternalServerError.into_response(),
    }
}
