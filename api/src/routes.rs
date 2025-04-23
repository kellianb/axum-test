use axum::middleware;
mod login_routes;
mod message_routes;
mod user_routes;

pub fn get_router() -> axum::Router {
    axum::Router::new()
        // These routes require authentication
        .nest("/users", user_routes::get_routes())
        .nest("/messages", message_routes::get_routes())
        .layer(middleware::from_fn(
            crate::layers::authentication::authenticate_request,
        ))
        // These routes do not require authentication
        .nest("/login", login_routes::get_routes())
}
