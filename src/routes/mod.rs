use axum::{body::Body, middleware, routing::{get, patch, post, put}, Extension, Router};
mod content;
mod user;
mod page;
mod image;
mod custom_middleware;

    // remove this (for debugging )

use tower_http::trace::{self, TraceLayer};
use tracing::Level;
//user
use user::user::{login, register,logout};
//content
use content::{create_content::create_content, get_content::{get_all_content, get_single_content}};

//page
use page::{create_page::create_page, get_page::{get_all_page, get_single_page}, update_page::atomic_update_page,partial_update_page::partial_update_page};

use image::{create_image::create_image, get_image::{get_all_image, get_single_image}, update_image::atomic_update_image,partial_update_image::partial_update_image};

use custom_middleware::authguard::authguard;

use sea_orm::DatabaseConnection;

pub fn create_routes(database: DatabaseConnection) -> Router<Body> {
    Router::new()
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn(authguard))

        //content
        .route("/content/:id", get(get_single_content))
        .route("/content/", get(get_all_content))

        .route("/content", post(create_content))

        //users
        .route("/register", post(register))
        .route("/login", post(login))
        //page 
        .route("/page", post(create_page))
        .route("/page", get(get_all_page))
        .route("/page/:id", get(get_single_page))
        .route("/page/:id", put(atomic_update_page))
        .route("/page/:id", patch(partial_update_page))

        //image 
        .route("/image", post(create_image))
        .route("/image", get(get_all_image))
        .route("/image/:id", get(get_single_image))
        .route("/image/:id", put(atomic_update_image))
        .route("/image/:id", patch(partial_update_image))
       

        .layer(Extension(database))
        // remove this (for debugging )

        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)),
                )
}