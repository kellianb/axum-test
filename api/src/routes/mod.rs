mod login_routes;
mod message_routes;
mod user_routes;

pub fn get_router() -> axum::Router {
    axum::Router::new()
        .nest("/users", user_routes::get_routes())
        .nest("/messages", message_routes::get_routes())
        .nest("/login", login_routes::get_routes())
}
