use axum::{middleware, Extension};
use elasticsearch::{http::transport::Transport, Elasticsearch};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
mod handlers;
mod layers;
mod models;
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

    let server_address = std::env::var("LISTEN_ADDRESS").unwrap_or("127.0.0.1:3000".to_string());

    let elastic_transport =
        Transport::single_node("http://localhost:9200").expect("Elasticsearch connection failed");
    let elastic_client = Arc::new(Elasticsearch::new(elastic_transport));

    let app = routes::get_router()
        .layer(Extension(get_db_pool().await))
        .layer(middleware::from_fn_with_state(
            elastic_client,
            layers::logging::log_request,
        ));

    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
