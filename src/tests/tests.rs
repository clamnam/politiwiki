use axum::{
    body::Body,
    extract::connect_info::MockConnectInfo,
    http::{Request, StatusCode},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use serde_json::Value;
use std::net::SocketAddr;
use tower::ServiceExt;

use crate::routes::create_routes;

pub async fn setup_test_app() -> Router {
    // Use an in-memory SQLite database for testing
    let database = Database::connect("sqlite::memory:").await.unwrap();
    
    // Initialize your test database here if needed
    // init_test_db(&database).await;
    
    create_routes(database)
}

pub async fn send_request(
    app: Router,
    method: &str,
    uri: &str,
    body: Option<String>,
    token: Option<&str>,
) -> (StatusCode, Option<Value>) {
    let mut req_builder = Request::builder()
        .method(method)
        .uri(uri);
    
    // Add auth token if provided
    if let Some(t) = token {
        req_builder = req_builder.header("Authorization", format!("Bearer {}", t));
    }
    
    let req = if let Some(body_content) = body {
        req_builder
            .header("Content-Type", "application/json")
            .body(Body::from(body_content))
            .unwrap()
    } else {
        req_builder
            .body(Body::empty())
            .unwrap()
    };
    
    // Process the request
    let response = app
        .oneshot(req)
        .await
        .unwrap();
    
    let status = response.status();
    
    // Parse the body if it exists
    let body = hyper::body::to_bytes(response.into_body())
        .await
        .ok()
        .and_then(|bytes| {
            if bytes.is_empty() {
                None
            } else {
                serde_json::from_slice(&bytes).ok()
            }
        });
        
    (status, body)
}