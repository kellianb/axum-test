use crate::error::AppError;
use crate::handlers::message_handlers;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use models::{login_models::LoginSession, message_models::*};
use serde_json::json;

pub fn get_routes() -> Router {
    axum::Router::new()
        .route("/", axum::routing::get(get_all))
        .route("/:id", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::delete(delete))
}

pub async fn get_all(Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>) -> impl IntoResponse {
    let response = message_handlers::get_all(&pool).await;

    match response {
        Ok(val) => (StatusCode::OK, Json(&val)).into_response(),
        Err(_) => AppError::InternalServerError.into_response(),
    }
}

pub async fn get(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    let response = message_handlers::get(id, &pool).await;

    match response {
        Ok(val) => (StatusCode::OK, Json(&val)).into_response(),
        Err(sqlx::Error::RowNotFound) => AppError::MessageNotFound.into_response(),
        Err(_) => AppError::InternalServerError.into_response(),
    }
}

pub async fn create(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
    Extension(session): Extension<LoginSession>,
    Json(message_data): Json<SendMessage>,
) -> impl IntoResponse {
    let response = message_handlers::create(&message_data, &session.user_id, &pool).await;

    match response {
        Ok(val) => (StatusCode::OK, Json(val)).into_response(),
        _ => AppError::InternalServerError.into_response(),
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    Extension(session): Extension<LoginSession>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    // Check if the current user is the sender
    match message_handlers::get(id, &pool).await {
        Ok(val) if val.sender_id == session.user_id => (),
        Ok(_) => return AppError::PermissionDenied.into_response(),
        Err(sqlx::Error::RowNotFound) => return AppError::MessageNotFound.into_response(),
        Err(_) => return AppError::InternalServerError.into_response(),
    };

    match message_handlers::delete(id, &pool).await {
        Ok(val) if val > 0 => (
            StatusCode::OK,
            Json(json!({ "message" : "Message deleted"})),
        )
            .into_response(),
        Ok(_) => AppError::MessageNotFound.into_response(),
        Err(_) => AppError::InternalServerError.into_response(),
    }
}
