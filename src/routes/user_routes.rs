use crate::handlers::user_handlers;
use crate::models::user_models::*;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    Router,
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
        Ok(val) => (StatusCode::OK, serde_json::to_string(&val).unwrap()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{ "message" : "Error" }"#.to_string(),
        ),
    }
}

pub async fn get(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    let response = user_handlers::get(id, &pool).await;

    match response {
        Ok(val) => (StatusCode::OK, serde_json::to_string(&val).unwrap()),
        Err(_) => (
            StatusCode::NOT_FOUND,
            r#"{ "message" : "User not found" }"#.into(),
        ),
    }
}

pub async fn create(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
    Json(user_data): Json<CreateUser>,
) -> impl IntoResponse {
    let response = user_handlers::create(user_data, &pool).await;

    match response {
        Ok(val) => (StatusCode::OK, serde_json::to_string(&val).unwrap()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{ "message" : "Error" }"#.to_string(),
        ),
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    let response = user_handlers::delete(id, &pool).await;

    match response {
        Ok(val) if val > 0 => (StatusCode::OK, r#"{ "message" : "User deleted"}"#),
        Ok(_) => (StatusCode::NOT_FOUND, r#"{ "message" : "User not found" }"#),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{ "message" : "Error" }"#,
        ),
    }
}
