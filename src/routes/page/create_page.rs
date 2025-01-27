
use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::database::pages;

#[derive(serde::Deserialize)]
pub struct PageCreate {
    title: String
}

pub async fn create_page(Extension(database): Extension<DatabaseConnection>,Json(request_page): Json<PageCreate>) {
    let new_pages = pages::ActiveModel {
        title: Set(request_page.title),
        created_at: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };
    let result = new_pages.save(&database).await.unwrap();
    dbg!(result);
}