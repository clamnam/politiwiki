#[cfg(test)]
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use hyper;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};
use serde_json::{json, Value};
use tower::ServiceExt;

use crate::routes::create_routes;

// Helper function to set up a test app
#[cfg(test)]
async fn setup_test_app() -> Router {
    let database = Database::connect("sqlite::memory:").await.unwrap();

    // Initialize test database
    init_test_db(&database).await.unwrap();

    create_routes(database)
}

// Helper function to send requests to the test app
#[cfg(test)]
async fn send_request(
    app: Router,
    method: &str,
    uri: &str,
    body: Option<String>,
    auth_token: Option<&str>,
) -> (StatusCode, Option<Value>) {
    let mut req = Request::builder().method(method).uri(uri);

    // Add auth token if provided
    if let Some(token) = auth_token {
        req = req.header("Authorization", format!("Bearer {}", token));
    }

    // Add body if provided
    let req = if let Some(body_content) = body {
        req.header("Content-Type", "application/json")
            .body(Body::from(body_content))
            .unwrap()
    } else {
        req.body(Body::empty()).unwrap()
    };

    // Process the request
    let response = app.oneshot(req).await.unwrap();
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

#[tokio::test]
async fn test_user_register() {
    let app = setup_test_app().await;

    // Test successful register
    let register_payload = json!({
        "email": "test@example.com",
        "username":"username",
        "password": "password123"
    })
    .to_string();

    let (status, body) = send_request(
        app.clone(),
        "POST",
        "/register",
        Some(register_payload),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_some());
    assert!(body.unwrap().get("token").is_some());
}

// Authentication tests
#[tokio::test]
async fn test_user_login() {
    let app = setup_test_app().await;

    // Test successful login
    let login_payload = json!({
        "email": "test@example.com",
        "password": "password123"
    })
    .to_string();

    let (status, body) =
        send_request(app.clone(), "POST", "/login", Some(login_payload), None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_some());
    assert!(body.unwrap().get("token").is_some());
}

#[tokio::test]
async fn test_user_logout() {
    // Test implementation here
}

// Content tests
#[tokio::test]
async fn test_create_content() {
    // Test implementation here
}

#[tokio::test]
async fn test_get_content() {
    // Test implementation here
}

// Add this function to your tests.rs file
#[cfg(test)]
async fn init_test_db(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Create tables for testing
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role_id INTEGER
        )",
    ))
    .await?;

    // Insert test user
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "INSERT INTO users (username, email, password, role_id) VALUES 
         ('testuser', 'test@example.com', '$2b$10$test_password_hash', 2)",
    ))
    .await?;

    Ok(())
}
