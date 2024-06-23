mod user_routes;

pub fn get_router() -> axum::Router {
    axum::Router::new().nest("/users", user_routes::get_routes())
}
