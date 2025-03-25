use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use axum::http::StatusCode;

use crate::database::pages;

#[derive(serde::Deserialize)]
pub struct PageCreate {
    title: String,
    page_type: i32
}

#[derive(serde::Serialize)]
pub struct PageCreateResponse {
    id: i32,
}

pub async fn create_page(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_page): Json<PageCreate>
) -> Result<Json<PageCreateResponse>, StatusCode> {
    let new_pages = pages::ActiveModel {
        title: Set(request_page.title),
        page_type: Set(Some(request_page.page_type)),
        created_at: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };
    
    match new_pages.save(&database).await {
        Ok(model) => {
            // Return the ID of the newly created page
            Ok(Json(PageCreateResponse { id: model.id.unwrap() }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}