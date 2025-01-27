use axum::{body::Body, routing::{get, post}, Extension, Router};
mod content;
mod user;
mod page;

//user
use user::user_create::user_create;

//content
use content::get_content::get_content;

//page
use page::{create_page::create_page, get_page::{get_single_page,get_all_pages}};

use sea_orm::DatabaseConnection;

pub fn create_routes(database: DatabaseConnection) -> Router<Body> {
    Router::new()
        .route("/content", get(get_content))
        // .route("/content", post(content_post))
        .route("/user", post(user_create))
        //page
        .route("/page", post(create_page))
        .route("/page", get(get_all_pages))
        .route("/page/:id", get(get_single_page))

        .layer(Extension(database))
}