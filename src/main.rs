use axum::Extension;
use sqlx::{Pool, Postgres};
mod routes;

async fn get_db_pool() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("Missing Database URL");

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Unable to connect to db")
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Missing .env file!");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_string());

    let app = routes::get_router().layer(Extension(get_db_pool().await));

    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
