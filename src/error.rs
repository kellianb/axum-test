use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Clone, Debug)]
pub enum AppError {
    InternalServerError,

    // User Errors
    UserNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let mut response = match self {
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
        .into_response();

        response.extensions_mut().insert(self);

        response
    }
}
