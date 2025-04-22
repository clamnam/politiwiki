use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::{Extension, Json, TypedHeader};

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::sea_orm_active_enums;
use crate::database::users::{self, Entity as Users};

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
    title: String,
    content_type: i32,
    content_body: String,
    images_id: Option<i32>,
    modified_by_id: Option<i32>,
    order_id: i32,
    page_id: i32,

}

pub async fn create_content(
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_content): Json<RequestContent>,
) -> Result<impl IntoResponse, StatusCode> {
    let token: &str = authorization.token();
    let user = match Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let request_image = images::Entity::find_by_id(request_content.images_id.unwrap_or_default())
        .one(&database)
        .await
        .map_err(|_| StatusCode::NOT_FOUND);
    let checked_image_id = if let Ok(Some(image)) = request_image {
        Some(image.id)
    } else {
        None
    };


    // Validate page_id by ensuring the page exists, returning a 406 if it does not.
    let checked_page_id = match crate::database::pages::Entity::find_by_id(request_content.page_id).one(&database).await {
        Ok(Some(page)) => Some(page.id),
        _ => return Err(StatusCode::NOT_ACCEPTABLE),
    };

    // Process queue field by creating a JSON representation of all fields
    let queue_entry = serde_json::json!({//stopped identifying json macro directly>?
        "title": request_content.title,
        "content_type": request_content.content_type,
        "content_body": request_content.content_body,
        "images_id": checked_image_id,
        "created_by_id": user.id,
        "modified_by_id": request_content.modified_by_id,
        "status": "Pending",
        "order_id": request_content.order_id,
        "page_id": checked_page_id,
        "is_deleted": false,
        "is_hidden": false,
        "created_at": Utc::now().naive_utc().to_string()
    });

    // Create an array containing the queue entry ,... siplifies
    let queue_json = Some(serde_json::json!([queue_entry]));
    let empty_history = Some(serde_json::json!([]));

    let new_contents = contents::ActiveModel {
        title: Set(request_content.title),
        content_type: Set(request_content.content_type),
        content_body: Set(request_content.content_body),
        images_id: Set(checked_image_id),
        created_by_id: Set(user.id),
        modified_by_id: Set(request_content.modified_by_id),
        status: Set(sea_orm_active_enums::Status::Pending),
        order_id: Set(Some(request_content.order_id)),
        page_id: Set(checked_page_id.unwrap_or_default()),
        queue: Set(queue_json),
        history: Set(empty_history),
        is_deleted: Set(false),
        is_hidden: Set(false),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let result = new_contents.save(&database).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    dbg!(result);
    Ok((StatusCode::CREATED, Json("Content created")))
}

