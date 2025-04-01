use axum::http::StatusCode;
use axum::Json;
use axum::{extract::Path, Extension, headers::{authorization::Bearer, Authorization}, TypedHeader};
use chrono::Utc;
use sea_orm::{sea_query, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set};
use crate::database::content::{self, Entity as Contents};
use crate::database::pages::Entity as Pages;

use crate::database::users::{self, Entity as Users};

use crate::database::sea_orm_active_enums::Status;
use axum::response::IntoResponse;
use json;
use sea_query::Expr;

#[derive(serde::Deserialize,Debug)]
pub struct RequestContent {
    queue_index: usize,  // Index of the queue item to approve
    modified_by_id: Option<i32>,  // Who is approving the content
}

pub async fn approve_content(
    authorization: TypedHeader<Authorization<Bearer>>,
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_content): Json<RequestContent>
) -> impl IntoResponse {

    // 1. Check if user is authorized (role >= 5)
    let token = authorization.token();
    let user = match Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };

    // Get the user's role
    let role = match user.find_related(crate::database::roles::Entity)
        .one(&database)
        .await
    {

        Ok(Some(role)) => role,
        Ok(None) => return StatusCode::FORBIDDEN,  // User has no role
        Err(err) => {
            dbg!(err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // Check if role is at least 5

    if role.title.unwrap_or(0) < 5 {
        return StatusCode::FORBIDDEN;
    }
    
    let temp = match Pages::find_by_id(id)
    .one(&database)
    .await
{
    Ok(Some(content)) => content,
    Ok(None) => return StatusCode::NOT_FOUND,
    Err(err) => {
        dbg!(err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }    };
    // dbg!(&temp);

    let current_content = match temp.find_related(Contents)
        .filter(content::Column::PageId.eq(id))
        // Check that the 'queue' JSON array has at least one element
        .filter(Expr::cust("json_array_length(queue) > 0"))
        .one(&database)
        .await
    {
        Ok(Some(content)) => content,
        Ok(None) => return StatusCode::NOT_FOUND,
        Err(err) => {
            dbg!(err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    
    // Get the actual content ID that needs to be updated
    let content_id = current_content.id;
    
    // 3. Parse the queue from the content
    let queue_json_str = match &current_content.queue {
        Some(queue) => queue.to_string(),
        None => return StatusCode::BAD_REQUEST, // No queue exists
    };

    let queue_parsed = match json::parse(&queue_json_str) {
        Ok(parsed) if parsed.is_array() => parsed,
        _ => return StatusCode:: BAD_REQUEST, 
    };
    dbg!("{:?},{:?}", &request_content.queue_index,&queue_parsed);

    // 4. Check if the requested queue index exists
    if request_content.queue_index >= queue_parsed.len() {
        return StatusCode::BAD_REQUEST;

    }
    // 5. Get the queue item to be approved
    let queue_item = &queue_parsed[request_content.queue_index];
    
    // 6. Create a history entry for the current state
    let history_entry = json::object!{
        "title": current_content.title.clone(),
        "content_type": current_content.content_type,
        "content_body": current_content.content_body.clone(),
        "images_id": current_content.images_id,
        "created_by_id": current_content.created_by_id,
        "modified_by_id": current_content.modified_by_id,
        "status": current_content.status.map(|s| format!("{:?}", s)).unwrap_or_default(),
        "order_id": current_content.order_id,
        "page_id": current_content.page_id,
        "is_deleted": current_content.is_deleted,
        "is_hidden": current_content.is_hidden,
        "updated_at": current_content.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
        "overwritten_at": Utc::now().naive_utc().to_string()
    };
    
    // 7. Prepare history JSON
    let mut history_array = json::JsonValue::new_array();
    if let Some(history) = &current_content.history {
        // Parse existing history
        if let Ok(parsed) = json::parse(&history.to_string()) {
            if parsed.is_array() {
                history_array = parsed;
            }
        }
    }
    
    // Add new history entry
    history_array.push(history_entry).unwrap_or(());
    
    // Convert to serde_json for sea_orm
    let history_json_string = history_array.dump();
    let history_serde_json: serde_json::Value = serde_json::from_str(&history_json_string)
        .unwrap_or(serde_json::Value::Array(vec![]));
    
    // 8. Create a new queue with the approved item removed
    let mut new_queue = json::JsonValue::new_array();
    for (index, item) in queue_parsed.members().enumerate() {
        if index != request_content.queue_index {
            new_queue.push(item.clone()).unwrap_or(());
        }
    }
    
    let queue_json_string = new_queue.dump();
    let queue_serde_json: serde_json::Value = serde_json::from_str(&queue_json_string)
        .unwrap_or(serde_json::Value::Array(vec![]));
    
    // 9. Update the content with approved changes
    let update_content = content::ActiveModel {
        id: Set(content_id),  // Use the correct content ID instead of page ID
        title: Set(queue_item["title"].as_str().map(|s| s.to_string())),
        content_type: Set(queue_item["content_type"].as_i32()),
        content_body: Set(queue_item["content_body"].as_str().map(|s| s.to_string())),
        images_id: Set(queue_item["images_id"].as_i32()),
        created_by_id: Set(queue_item["created_by_id"].as_i32()),
        modified_by_id: Set(request_content.modified_by_id.or(current_content.modified_by_id)),
        page_id: Set(queue_item["page_id"].as_i32()),
        status: Set(Some(Status::Published)),  // Set status to Published
        order_id: Set(queue_item["order_id"].as_i32()),
        is_hidden: Set(queue_item["is_hidden"].as_bool().or(current_content.is_hidden)),
        is_deleted: Set(queue_item["is_deleted"].as_bool().or(current_content.is_deleted)),
        created_at: Set(current_content.created_at),
        updated_at: Set(Some(Utc::now().naive_utc())),
        history: Set(Some(history_serde_json)),
        queue: Set(Some(queue_serde_json)),
    };
    
    // 10. Save to database
    match Contents::update(update_content)
        .filter(content::Column::Id.eq(content_id))  // Filter by content ID, not page ID
        .exec(&database)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            dbg!(err, "Error updating content");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}


