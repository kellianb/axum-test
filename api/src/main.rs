use axum::{middleware, Extension};
use elasticsearch::{http::transport::Transport, Elasticsearch};
use models::login_models::LoginSession;
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
mod error;
mod handlers;
mod layers;
mod routes;

async fn get_db_pool() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Database URL, example: postgresql://username:password@localhost:5432/main");

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Unable to connect to db")
}

#[tokio::main]
async fn main() {
    // Get Environment
    let server_address = std::env::var("LISTEN_ADDRESS")
        .expect("An address for the server to listen on, example: 127.0.0.1:3000");

    // ---- Elastic ----
    let elastic_node_url = std::env::var("ELASTIC_NODE_URL")
        .expect("Url to an Elasticsearch node, example: http://localhost:9200");

    // Setup elastic
    let elastic_transport =
        Transport::single_node(&elastic_node_url).expect("Elasticsearch connection failed");

    let elastic = Elasticsearch::new(elastic_transport);

    // ---- CORS ----
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin
        .allow_methods(Any) // Allow any HTTP method
        .allow_headers(Any); // Allow any header

    // ---- App ----
    // Build app stack
    let app = routes::get_router()
        .layer(middleware::from_fn(
            layers::authentication::authenticate_request,
        ))
        .layer(middleware::from_fn(layers::logging::log_request))
        .layer(Extension(elastic))
        .layer(Extension(get_db_pool().await))
        .layer(cors);

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
