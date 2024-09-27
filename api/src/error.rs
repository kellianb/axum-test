use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum AppError {
    InternalServerError, // Catch all for unknown errors

    // User Errors
    UserNotFound,    // There is no user with the specified ID
    InvalidUserRole, // The given role does not exist

    // Message errors
    MessageNotFound, // The is no message with the specified ID
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
            Self::InvalidUserRole => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "message" : "The given role_id does not correspond to any existing role" })),
            ),
            Self::MessageNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"message" : "Message not found"}))
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
