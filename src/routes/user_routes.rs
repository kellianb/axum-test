use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use chrono::{NaiveDateTime, Utc};
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
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct NewUser {
    pub id: i32,
}

pub fn get_routes() -> Router {
    axum::Router::new()
        .route("/", axum::routing::get(get_all))
        .route("/:id", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::delete(delete))
}

pub async fn get(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, role, created_at FROM main.users WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await;

    match user {
        Ok(val) => (StatusCode::OK, serde_json::to_string(&val).unwrap()),
        Err(_) => (
            StatusCode::NOT_FOUND,
            r#"{ "message" : "User not found" }"#.into(),
        ),
    }
}

pub async fn get_all(Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>) -> impl IntoResponse {
    let query = sqlx::query_as!(
        User,
        "SELECT id, username, role, created_at FROM main.users"
    )
    .fetch_all(&pool)
    .await;

    match query {
        Ok(val) => (StatusCode::OK, serde_json::to_string(&val).unwrap()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{ "message" : "Error" }"#.to_string(),
        ),
    }
}

pub async fn create(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
    Json(user_data): Json<CreateUser>,
) -> impl IntoResponse {
    let query = sqlx::query_as!(
        NewUser,
        r#"INSERT INTO main.users (id, username, role, created_at) 
        VALUES (DEFAULT, $1, $2, $3)
        RETURNING id"#,
        user_data.username,
        user_data.role,
        Utc::now().naive_utc(),
    )
    .fetch_one(&pool)
    .await;

    match query {
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
    let query = sqlx::query!("DELETE FROM main.users WHERE id = $1", id)
        .execute(&pool)
        .await;

    match query {
        Ok(val) if val.rows_affected() > 0 => (StatusCode::OK, r#"{ "message" : "User deleted"}"#),
        Ok(_) => (StatusCode::NOT_FOUND, r#"{ "message" : "User not found" }"#),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{ "message" : "Error" }"#,
        ),
    }
}
