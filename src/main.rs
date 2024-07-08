use axum::{middleware, Extension};
use elasticsearch::{http::transport::Transport, Elasticsearch};
use sqlx::{Pool, Postgres};
mod error;
mod handlers;
mod layers;
mod models;
mod routes;

async fn get_db_pool() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect(
        "Missing Database URL, example: postgresql://username:password@localhost:5432/main",
    );

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Unable to connect to db")
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Missing .env file!");

    // Get Environment
    let server_address = std::env::var("LISTEN_ADDRESS")
        .expect("An address for the server to listen on, example: 127.0.0.1:3000");

    let elastic_node_url = std::env::var("ELASTIC_NODE_URL")
        .expect("Url to an Elasticsearch node, example: http://localhost:9200");

    // Setup elastic
    let elastic_transport =
        Transport::single_node(&elastic_node_url).expect("Elasticsearch connection failed");

    let elastic = Elasticsearch::new(elastic_transport);

    // Build app stack
    let app = routes::get_router()
        .layer(middleware::from_fn(layers::logging::log_request))
        .layer(Extension(elastic))
        .layer(Extension(get_db_pool().await));

    // Run app
    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
