use axum::{body::Body, routing::{get, patch, post, put}, Extension, Router};
mod content;
mod user;
mod page;

//user
use user::user::{login, register};

//content
use content::get_content::get_content;

//page
use page::{create_page::create_page, get_page::{get_all_page, get_single_page}, update_page::atomic_update_page,partial_update_page::partial_update_page};

use sea_orm::DatabaseConnection;

pub fn create_routes(database: DatabaseConnection) -> Router<Body> {
    Router::new()
        .route("/content", get(get_content))
        // .route("/content", post(content_post))
        .route("/user/register", post(register))
        .route("/user/login", post(login))

        //page
        .route("/page", post(create_page))
        .route("/page", get(get_all_page))
        .route("/page/:id", get(get_single_page))
        .route("/page/:id", put(atomic_update_page))
        .route("/page/:id", patch(partial_update_page))

        .layer(Extension(database))
}