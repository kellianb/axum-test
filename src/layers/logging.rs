use chrono::prelude::*;

use axum::{
    extract::{connect_info::ConnectInfo, Extension, Request},
    http::header::{HOST, USER_AGENT},
    middleware::Next,
    response::Response,
};
use elasticsearch::{Elasticsearch, IndexParts};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    method: String,
    uri: String,
    host: String,
    user_agent: String,
}

pub async fn log_request(
    ConnectInfo(socket_address): ConnectInfo<std::net::SocketAddr>,
    Extension(client): Extension<Elasticsearch>,
    request: Request,
    next: Next,
) -> Response {
    let headers = request.headers();
    let socket_ip = socket_address.ip();
    let socket_port = socket_address.port();

    let mut log = json!( {
        "method": format!("{:?}", request.method()),
        "uri": request.uri().to_string(),
        "user_agent": headers[USER_AGENT].to_str().unwrap_or(""),
        "socket_ip": socket_ip,
        "socket_port": socket_port,
        "host": headers[HOST].to_str().unwrap_or(""),
        "time": Utc::now().timestamp_millis(),
    });

    // Execute request
    let response = next.run(request).await;

    // Log status code
    log.as_object_mut()
        .unwrap()
        .insert("status_code".to_string(), json!(response.status().as_u16()));

    let elastic_index = format! {"api-log-{}", Utc::now().format("%Y-%m")};

    let log_response = client
        .index(IndexParts::Index(&elastic_index))
        .body(log)
        .send()
        .await;

    if let Err(val) = log_response {
        println!("Logging error!");
        println!("{:?}", val);
    }

    // Return response
    response
}
