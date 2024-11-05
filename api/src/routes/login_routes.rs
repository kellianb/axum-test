use crate::error::AppError;
use crate::handlers::login_handlers;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use chrono::{Duration, Utc};
use models::login_models::*;
use rand::Rng;
use serde_json::json;

pub fn get_routes() -> Router {
    axum::Router::new().route("/", axum::routing::post(create))
}

pub async fn create(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
    Json(create_login_data): Json<CreateLogin>,
) -> impl IntoResponse {
    // Fetch password hash and user id from DB
    let login_data = match login_handlers::get_login_data(&create_login_data.username, &pool).await
    {
        Ok(data) => data,
        Err(sqlx::Error::RowNotFound) => return AppError::IncorrectCredentials.into_response(),
        Err(_) => return AppError::InternalServerError.into_response(),
    };

    // Parse password hash
    let password_hash = match PasswordHash::new(&login_data.password_hash) {
        Ok(data) => data,
        Err(_) => return AppError::InternalServerError.into_response(),
    };

    // Verify password
    if Argon2::default()
        .verify_password(&create_login_data.password.into_bytes(), &password_hash)
        .is_err()
    {
        return AppError::IncorrectCredentials.into_response();
    }

    // Generate session token
    let token: String = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();

    // Create session struct
    let session = LoginSession {
        created_at: Utc::now().naive_utc(),
        expires_at: Utc::now().naive_utc() + Duration::days(2),
        user_id: login_data.id,
        token,
    };

    // Create session in DB
    if login_handlers::create(&session, &pool).await.is_err() {
        return AppError::InternalServerError.into_response();
    }

    let session = if let Ok(data) = serde_json::to_string(&session) {
        data
    } else {
        return AppError::InternalServerError.into_response();
    };

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "Set-Cookie",
        format!("x-session-data={session}").parse().unwrap(),
    );

    (
        StatusCode::CREATED,
        headers,
        Json(json!({ "message" : "Session created"})),
    )
        .into_response()
}
