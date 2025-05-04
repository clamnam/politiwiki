mod database;
mod routes;
mod utils;
use axum::http;
use routes::create_routes;
use sea_orm::Database;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use http::{Method, header,HeaderValue};
use std::env;

// Make tests a proper module with conditional compilation
#[cfg(test)]
pub mod tests;

pub async fn run(database_url: &str) {
    let database: sea_orm::DatabaseConnection = Database::connect(database_url).await.unwrap();
    
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    println!("Allowed frontend URL: {}", frontend_url);

    // Create a CORS layer that will apply to all routes
    let cors = CorsLayer::new()
        // Allow requests from your frontend
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    // Build router with routes
    let app = create_routes(database)
        // Apply the CORS layer globally to all routes
        .layer(cors);
    
    // Set up server
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
