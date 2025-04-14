use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::{Extension, Json, TypedHeader};

use axum::extract::Path;
use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::content::{self, Entity as Contents};
use crate::database::users::{self, Entity as Users};
// Add this import for the Status enum
// use crate::database::sea_orm_active_enums::Status;
use json;

// pub enum StatusValues {
//     Pending,
//     Approved,
//     Rejected,
//     Published,
// }

#[derive(serde::Deserialize)]
pub struct RequestContent {
    title: String,
    content_body: String,
    images_id: Option<i32>,
    created_by_id: Option<i32>,
    order_id: Option<i32>,
    page_id: Option<i32>,
}

pub async fn queue_partial_update_content(
    authorization: TypedHeader<Authorization<Bearer>>,

    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_content): Json<RequestContent>
) -> impl IntoResponse {
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
    let current = match Contents::find_by_id(id)
        .one(&database)
        .await {
            Ok(Some(content)) => content,
            Ok(None) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
        
    // Check if page_id and images_id exists in the request or use current values
    let checked_page_id = request_content.page_id.or(Some(current.page_id));
    let checked_image_id = request_content.images_id.or(current.images_id);

    // Create new queue entry
    let new_queue_entry = json::object!{
        "title": request_content.title.clone(),
        "content_type": 1,
        "content_body": request_content.content_body.clone(),
        "images_id": checked_image_id,
        "created_by_id": request_content.created_by_id,
        "modified_by_id": Utc::now().naive_utc().to_string(),
        "status": "Pending",
        "modified_by_id": user.id,
        "order_id": request_content.order_id,
        "page_id": checked_page_id,
        "is_deleted": false,
        "is_hidden": false,
        "created_at": current.created_at.to_string()
    };

    // Initialize queue entries
    let mut queue_entries = json::JsonValue::new_array();
    // Try to parse existing queue from sea_orm::JsonValue
    if let Some(db_json) = &current.queue {
        dbg!(&db_json,"current queue");
        // Convert sea_orm::JsonValue to a string we can parse with json crate
        let json_str = db_json.to_string();
        if !json_str.is_empty() && json_str != "null" {
            match json::parse(&json_str) {
                Ok(json_val) if json_val.is_array() => {
                    queue_entries = json_val;
                    dbg!(&queue_entries,"queue_entries");
                },
                _ => {} // Keep empty array
            }
        }
    }

    // Add new entry to queue
    queue_entries.push(new_queue_entry).unwrap_or(());

    // Convert to serde_json::Value for sea_orm
    let queue_json_string = queue_entries.dump();
    let queue_serde_json: serde_json::Value = serde_json::from_str(&queue_json_string)
        .unwrap_or(serde_json::Value::Array(vec![]));

    // Update content with new queue
    let update_content = content::ActiveModel {
        id: Set(id),
        title: Set(current.title),
        created_at: Set(current.created_at),
        updated_at: Set(Some(Utc::now().naive_utc())),
        content_type: Set(current.content_type),
        content_body: Set(current.content_body),
        images_id: Set(current.images_id),
        created_by_id: Set(current.created_by_id),
        modified_by_id: Set(current.modified_by_id),
        page_id: Set(current.page_id),
        status: Set(current.status),
        order_id: Set(current.order_id),
        is_hidden: Set(current.is_hidden),
        is_deleted: Set(current.is_deleted),
        history: Set(current.history),
        queue: Set(Some(queue_serde_json)),
    };

    // Save to database
    match Contents::update(update_content)
        .filter(content::Column::Id.eq(id))
        .exec(&database)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

