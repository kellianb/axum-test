use chrono::prelude::*;
use std::sync::Arc;

use axum::{
    extract::{connect_info::ConnectInfo, Request, State},
    http::header::{HOST, USER_AGENT},
    middleware::Next,
    response::Response,
};
use elasticsearch::{Elasticsearch, IndexParts};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    method: String,
    uri: String,
    host: String,
    user_agent: String,
}

pub async fn log_request(
    State(client): State<Arc<Elasticsearch>>,
    ConnectInfo(socket_address): ConnectInfo<std::net::SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    //println!("{:?}", request);

    let headers = request.headers();
    let socket_ip = socket_address.ip();
    let socket_port = socket_address.port();

    let log = json!( {
        "method": format!("{:?}", request.method()),
        "uri": request.uri().to_string(),
        "user_agent": headers[USER_AGENT].to_str().unwrap_or(""),
        "socket_ip": socket_ip,
        "socket_port": socket_port,
        "host": headers[HOST].to_str().unwrap_or(""),
        "time": Utc::now().timestamp_millis(),
    });

    let index = format! {"api-log-{}", Utc::now().format("%Y-%m")};

    let response = client
        .index(IndexParts::Index(&index))
        .body(log)
        .send()
        .await;

    if let Err(val) = response {
        println!("Logging error!");
        println!("{:?}", val);
    }

    //println!("{:?}", response);

    next.run(request).await
}
