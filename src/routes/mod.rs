use axum::{body::Body, routing::post, Router};
mod content_get;
use content_get::content_get;
pub fn create_routes()->Router<Body>{
    Router::new()
        .route("/content", post(content_get))
}