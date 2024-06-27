use chrono::prelude::*;
use std::sync::Arc;

use axum::{
    extract::{Request, State},
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
    request: Request,
    next: Next,
) -> Response {
    //println!("{:?}", request);
    let headers = request.headers();

    let log = json!( {
        "method": format!("{:?}", request.method()),
        "uri": request.uri().to_string(),
        "user_agent": headers[USER_AGENT].to_str().unwrap_or("").to_string(),
        "host": headers[HOST].to_str().unwrap_or("").to_string(),
        "time": Utc::now().timestamp_millis(),
    });

    let index = format! {"api-log-{}", Utc::now().format("%Y-%m").to_string()};

    let response = client
        .index(IndexParts::Index(&index))
        .body(log)
        .send()
        .await;

    //println!("{:?}", response);

    next.run(request).await
}
