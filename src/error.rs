use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub enum Error {
    InternalServerError,

    // User Errors
    UserNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message" : "Internal server error" })),
            ),
            Self::UserNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({ "message" : "User not found" })),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message" : "Internal server error" })),
            ),
        }
        .into_response()
    }
}
