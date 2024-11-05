use axum::{
    extract::{Extension, Request},
    http::{HeaderMap, HeaderValue},
    middleware::Next,
    response::{IntoResponse, Response},
};
use models::login_models::LoginSession;

use chrono::Utc;

use crate::{error::AppError, handlers::*};

pub async fn authenticate_request(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    if request.uri().to_string().starts_with("/login") {
        return next.run(request).await;
    }

    // Try to fetch header value
    let session: &HeaderValue = if let Some(data) = headers.get("x-session-data") {
        data
    } else {
        return AppError::IncorrectCredentials.into_response();
    };

    // Try to convert header value to string
    let session = if let Ok(data) = session.to_str() {
        data
    } else {
        return AppError::IncorrectCredentials.into_response();
    };

    // Try to parse header value string into LoginSession struct
    let session: LoginSession = if let Ok(data) = serde_json::from_str(&session) {
        data
    } else {
        return AppError::IncorrectCredentials.into_response();
    };

    // Check if session exists in DB, check if it has expired
    match login_handlers::get_session(&session.token, &pool).await {
        Ok(val) => {
            if val.expires_at > Utc::now().naive_utc() {
                request.extensions_mut().insert(session);
                return next.run(request).await;
            }
            AppError::IncorrectCredentials.into_response()
        }
        Err(_) => AppError::IncorrectCredentials.into_response(),
    }
}
