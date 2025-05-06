mod database;
mod routes;
mod utils;
use axum::http;
use routes::create_routes;
use sea_orm::Database;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use http::{Method, header, HeaderValue};
use std::env;

// Make tests a proper module with conditional compilation
#[cfg(test)]
pub mod tests;

pub async fn run(database_url: &str) {
    let database: sea_orm::DatabaseConnection = Database::connect(database_url).await.unwrap();
    
    // Get the allowed frontend URL and parse it to HeaderValue
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    println!("Allowed frontend URL: {}", frontend_url);
    let allowed_origin = HeaderValue::from_str(&frontend_url)
        .expect("Invalid FRONTEND_URL provided");

    // Use a specific allowed origin instead of Any when credentials are allowed.
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    let app = create_routes(database)
        .layer(cors);
    
    let addr = SocketAddr::from(([0,0,0,0], 3000));
    println!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
