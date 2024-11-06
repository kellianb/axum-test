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
    // Try to fetch x-session-data header value
    let session: &HeaderValue = match headers.get("x-session-data") {
        Some(data) => data,
        None => return AppError::InvalidSession.into_response(),
    };

    // Try to convert header value to string
    let session = match session.to_str() {
        Ok(data) => data,
        Err(_) => return AppError::InvalidSession.into_response(),
    };

    // Try to parse header value string into LoginSession struct
    let session: LoginSession = match serde_json::from_str(session) {
        Ok(data) => data,
        Err(_) => return AppError::InvalidSession.into_response(),
    };

    // Check if session exists in DB, check if it has expired
    match login_handlers::get_session(&session.token, &pool).await {
        Ok(data) if data.expires_at > Utc::now().naive_utc() => {
            request.extensions_mut().insert(data);
            next.run(request).await
        }
        _ => AppError::InvalidSession.into_response(),
    }
}
