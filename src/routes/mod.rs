use axum::{body::Body, middleware, routing::{delete, get, patch, post, put}, Extension, Router};
mod content;
mod user;
mod page;
mod image;
mod custom_middleware;
mod admin;

// use tower_http::trace::{self, TraceLayer};
// use tracing::Level;
//user
use user::user::{login, register,logout};
//content
use content::{create_content::create_content, get_content::{get_all_content, get_single_content,get_content_by_page},update_content::queue_partial_update_content,delete_content::queue_delete_content};
use admin::approve_content::approve_content;
//page
use page::{create_page::create_page, get_page::{get_all_page, get_single_page}, update_page::atomic_update_page,partial_update_page::partial_update_page};

use image::{create_image::create_image, get_image::{get_all_image, get_single_image}, update_image::atomic_update_image,partial_update_image::partial_update_image};

use custom_middleware::authguard::authguard;

use sea_orm::DatabaseConnection;

pub fn create_routes(database: DatabaseConnection) -> Router<Body> {
    // Define routes that require authentication
    let auth_routes = Router::new()
        .route("/logout", post(logout))
        .route("/image", post(create_image))
        .route("/content", post(create_content))

        .route("/content/queue/:id", patch(queue_partial_update_content))
        .route("/content/queue/:id", delete(queue_delete_content))

        .route("/content/:id", patch(approve_content))

        .route("/page", post(create_page))
        .route("/page/:id", put(atomic_update_page))
        .route("/page/:id", patch(partial_update_page))
        .route("/image/:id", put(atomic_update_image))
        .route("/image/:id", patch(partial_update_image))
        .layer(middleware::from_fn(authguard));

    // Define public routes
    let public_routes = Router::new()
        .route("/content/:id", get(get_single_content))


        .route("/content/bypage/:id", get(get_content_by_page))
        .route("/content/", get(get_all_content))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/page", get(get_all_page))
        .route("/page/:id", get(get_single_page))
        .route("/image", get(get_all_image))
        .route("/image/:id", get(get_single_image));

    // Combine routes and add database extension
    Router::new()
        .merge(auth_routes)
        .merge(public_routes)
        .layer(Extension(database))
}