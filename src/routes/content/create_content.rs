use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use crate::database::sea_orm_active_enums;
use serde_json::json; // Add this import

use crate::database::{content as contents, images};
#[derive(serde::Deserialize)]
pub enum StatusValues {
    Pending,
    Approved,
    Rejected,
    Published,
}

#[derive(serde::Deserialize)]
pub struct RequestContent {
    title: Option<String>,
    content_type: Option<i32>,
    content_body: Option<String>,
    images_id: Option<i32>,
    created_by_id: Option<i32>,
    modified_by_id: Option<i32>,
    order_id: Option<i32>,
    page_id: Option<i32>,

}

pub async fn create_content(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_content): Json<RequestContent>,
) -> Result<impl IntoResponse, StatusCode> {
    let request_image = images::Entity::find_by_id(request_content.images_id.unwrap_or_default())
        .one(&database)
        .await
        .map_err(|_| StatusCode::NOT_FOUND);
    let checked_image_id = if let Ok(Some(image)) = request_image {
        Some(image.id)
    } else {
        None
    };


    // If a page_id is provided, return a 404 if the page does not exist.
    let checked_page_id = if let Some(page_id) = request_content.page_id {
        match crate::database::pages::Entity::find_by_id(page_id).one(&database).await {
            Ok(Some(page)) => Some(page.id),
            _ => return Err(StatusCode::NOT_ACCEPTABLE),
        }
    } else {
        None
    };

    // Process queue field by creating a JSON representation of all fields
    let queue_content = json!({
        "title": request_content.title,
        "content_type": request_content.content_type,
        "content_body": request_content.content_body,
        "images_id": checked_image_id,
        "created_by_id": request_content.created_by_id,
        "modified_by_id": request_content.modified_by_id,
        "status": "Pending",
        "order_id": request_content.order_id,
        "page_id": checked_page_id,
        "is_deleted": false,
        "is_hidden": false,
        "created_at": Utc::now().naive_utc().to_string()
    });

    // Use the JSON value directly instead of converting to a string
    let queue_json = Some(queue_content);

    let new_contents = contents::ActiveModel {
        title: Set(request_content.title),
        content_type: Set(request_content.content_type),
        content_body: Set(request_content.content_body),
        images_id: Set(checked_image_id),
        created_by_id: Set(request_content.created_by_id),
        modified_by_id: Set(request_content.modified_by_id),
        status: Set(Some(sea_orm_active_enums::Status::Pending)),
        order_id: Set(request_content.order_id),
        page_id: Set(checked_page_id),
        queue: Set(queue_json),
        is_deleted: Set(Some(false)),
        is_hidden: Set(Some(false)),
        created_at: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };

    let result = new_contents.save(&database).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    dbg!(result);
    Ok((StatusCode::CREATED, Json("Content created")))
}

