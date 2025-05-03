use crate::database::content::{self, Entity as Contents};
use crate::database::roles::{self, Entity as Role};

use crate::database::pages::{self, Entity as Pages};
use crate::utils::role::role_augment;
use axum::http::StatusCode;
use axum::Json;
use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    Extension, TypedHeader,
};
use chrono::Utc;
use sea_orm::{
    sea_query, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set,
};

use crate::database::users::{self, Entity as Users, Model};

use crate::database::sea_orm_active_enums::Status;
use axum::response::IntoResponse;
use json::{self, JsonValue};
use sea_query::Expr;

#[derive(serde::Deserialize, Debug)]
pub struct RequestContent {
    approval: bool,
    queue_index: usize,          // Index of the queue item to approve
    modified_by_id: Option<i32>, // Who is approving the content
    content_id: Option<i32>,     // Add this field to receive the content ID
}

pub async fn approve_content(
    authorization: TypedHeader<Authorization<Bearer>>,
    Path(_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_content): Json<RequestContent>,
) -> impl IntoResponse {
    // 1. Get user token
    let token = authorization.token();

    // 2. Find user based on token
    let user = match find_user(token, &database).await {
        Ok(user) => user,
        Err(status) => return status,
    };

    // 3. Get the user's role
    let role = match find_role(&database, user).await {
        Ok(role) => role,
        Err(status) => return status,
    };

    // 4. Check if role is at least 0.5, admin threshold
    if role.title < 0.5 {
        return StatusCode::UNAUTHORIZED;
    }

    // 6. Find the content for this page
    let content_id = match request_content.content_id {
        Some(id) => id,
        None => return StatusCode::BAD_REQUEST, // Handle missing content_id
    };

    let current_content = match find_content(&database, content_id).await {
        Ok(contents) => contents,
        Err(status) => return status,
    };

    // 7. Parse the queue from the content
    let queue_json_str = match &current_content.queue {
        Some(queue) => queue.to_string(),
        None => return StatusCode::BAD_REQUEST, // No queue exists
    };

    let queue_parsed = match json::parse(&queue_json_str) {
        Ok(parsed) if parsed.is_array() => parsed,
        _ => return StatusCode::BAD_REQUEST,
    };

    // 8. Check if the requested queue index exists
    if request_content.queue_index >= queue_parsed.len() {
        return StatusCode::BAD_REQUEST;
    }

    // 9. Get the queue item to be approved
    let queue_item = queue_parsed[request_content.queue_index].to_owned();

    // 10. case based on approval decision
    if &request_content.approval == &true {
        dbg!("approving");
        process_approval(
            &current_content,
            &queue_item,
            &queue_parsed,
            &request_content,
            role,
            &database,
        )
        .await
    } else {
        dbg!("denying");
        process_denial(
            &current_content,
            &queue_item,
            &queue_parsed,
            &request_content,
            role,
            &database,
        )
        .await
    }
}

async fn process_approval(
    current_content: &content::Model,
    queue_item: &JsonValue,
    queue_parsed: &JsonValue,
    request_content: &RequestContent,
    role: roles::Model,
    database: &DatabaseConnection,
) -> StatusCode {
    let content_id = current_content.id;

    // 1. Create a history entry for the current state
    let history_entry = json::object! {
        "title": current_content.title.clone(),
        "content_type": current_content.content_type,
        "content_body": current_content.content_body.clone(),
        "images_id": current_content.images_id,
        "created_by_id": current_content.created_by_id,
        "modified_by_id": current_content.modified_by_id,
        "status": match current_content.status {
            Status::Pending => "pending",
            Status::Approved => "approved",
            Status::Rejected => "rejected",
            Status::Published => "published",
        },
        "order_id": current_content.order_id,
        "page_id": current_content.page_id,
        "is_deleted": current_content.is_deleted,
        "is_hidden": current_content.is_hidden,
        "updated_at": current_content.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
        "overwritten_at": Utc::now().naive_utc().to_string()
    };

    // 2. Prepare history JSON
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
    let history_serde_json: serde_json::Value =
        serde_json::from_str(&history_json_string).unwrap_or(serde_json::Value::Array(vec![]));

    // 3. Create a new queue with the approved item removed
    let mut new_queue = json::JsonValue::new_array();
    for (index, item) in queue_parsed.members().enumerate() {
        if index != request_content.queue_index {
            new_queue.push(item.clone()).unwrap_or(());
        }
    }
    dbg!(queue_item);
    let queue_json_string = new_queue.dump();
    let queue_serde_json: serde_json::Value =
        serde_json::from_str(&queue_json_string).unwrap_or(serde_json::Value::Array(vec![]));

    // 4. Update the content with approved changes
    let update_content = content::ActiveModel {
        id: Set(content_id), // Use the correct content ID instead of page ID
        title: Set(queue_item["title"].as_str().unwrap_or_default().to_owned()),
        content_type: Set(queue_item["content_type"].as_i32().unwrap_or_default()),
        content_body: Set(queue_item["content_body"]
            .as_str()
            .unwrap_or_default()
            .to_owned()),
        images_id: Set(queue_item["images_id"].as_i32()),
        created_by_id: Set(queue_item["created_by_id"].as_i32().unwrap_or_default()),
        modified_by_id: Set(request_content
            .modified_by_id
            .or(current_content.modified_by_id)),
        page_id: Set(queue_item["page_id"].as_i32().unwrap_or_default()),
        status: Set(Status::Published), // Set status to Published
        order_id: Set(queue_item["order_id"].as_i32()),
        is_hidden: Set(queue_item["is_hidden"]
            .as_bool()
            .unwrap_or(current_content.is_hidden)),
        is_deleted: Set(queue_item["is_deleted"]
            .as_bool()
            .unwrap_or(current_content.is_deleted)),

        created_at: Set(current_content.created_at),
        updated_at: Set(Some(Utc::now().naive_utc())),
        history: Set(Some(history_serde_json)),
        queue: Set(Some(queue_serde_json)),
    };

    // 5. Save to database
    match Contents::update(update_content)
        .filter(content::Column::Id.eq(content_id))
        .exec(database)
        .await
    {
        Ok(_) => (),
        Err(err) => {
            dbg!(err, "Error updating content");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // 6. Update user role title based on approval
    let update_role_title = role_augment(role.title, true);

    let update_role = roles::ActiveModel {
        id: Set(role.id),
        title: Set(update_role_title),
    };

    match Role::update(update_role).exec(database).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            dbg!(err, "Error updating role");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn process_denial(
    current_content: &content::Model,
    queue_item: &JsonValue,
    queue_parsed: &JsonValue,
    request_content: &RequestContent,
    role: roles::Model,
    database: &DatabaseConnection,
) -> StatusCode {
    let content_id = current_content.id;

    // 1. Create history entry from queue item (for rejected content)
    let history_entry = json::object! {
        "title": queue_item["title"].clone(),
        "content_type": queue_item["content_type"].clone(),
        "content_body": queue_item["content_body"].clone(),
        "images_id": queue_item["images_id"].clone(),
        "created_by_id": queue_item["created_by_id"].clone(),
        "modified_by_id": queue_item["modified_by_id"].clone(),
        "status": "rejected",
        "order_id": queue_item["order_id"].clone(),
        "page_id": queue_item["page_id"].clone(),
        "is_deleted": queue_item["is_deleted"].clone(),
        "is_hidden": queue_item["is_hidden"].clone(),
        "updated_at": queue_item["updated_at"].clone(),
    };

    // 2. Prepare history JSON
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
    let history_serde_json: serde_json::Value =
        serde_json::from_str(&history_json_string).unwrap_or(serde_json::Value::Array(vec![]));

    // 3. Create a new queue with the denied item removed
    let mut new_queue = json::JsonValue::new_array();

    for (index, item) in queue_parsed.members().enumerate() {
        if index != request_content.queue_index {
            new_queue.push(item.clone()).unwrap_or(());
        }
    }

    let queue_json_string = new_queue.dump();
    let queue_serde_json: serde_json::Value =
        serde_json::from_str(&queue_json_string).unwrap_or(serde_json::Value::Array(vec![]));

    let update_content = content::ActiveModel {
        id: Set(content_id), // Use the correct content ID instead of page ID
        title: Set(current_content.title.clone()),
        content_type: Set(current_content.content_type),
        content_body: Set(current_content.content_body.clone()),
        images_id: Set(current_content.images_id),
        created_by_id: Set(current_content.created_by_id),
        modified_by_id: Set(current_content.modified_by_id),
        page_id: Set(current_content.page_id),
        status: Set(Status::Published), // Set status to Published
        order_id: Set(current_content.order_id),
        is_hidden: Set(current_content.is_hidden),
        is_deleted: Set(current_content.is_deleted),
        created_at: Set(current_content.created_at),
        updated_at: Set(current_content.updated_at),
        history: Set(Some(history_serde_json)),
        queue: Set(Some(queue_serde_json)),
    };

    // 5. Save to database
    match Contents::update(update_content)
        .filter(content::Column::Id.eq(content_id))
        .exec(database)
        .await
    {
        Ok(_) => (),
        Err(err) => {
            dbg!(err, "Error updating content");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // 6. Update user role title based on denial
    let update_role_title = role_augment(role.title, false);

    let update_role = roles::ActiveModel {
        id: Set(role.id),
        title: Set(update_role_title),
    };

    match Role::update(update_role).exec(database).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            dbg!(err, "Error updating role");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn find_user(token: &str, database: &DatabaseConnection) -> Result<users::Model, StatusCode> {
    match Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(database)
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn find_role(database: &DatabaseConnection, user: Model) -> Result<roles::Model, StatusCode> {
    let role = match user
        .find_related(crate::database::roles::Entity)
        .one(database)
        .await
    {
        Ok(Some(role)) => Ok(role),
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    role
}


async fn find_content(
    database: &DatabaseConnection,
    content_id: i32,
) -> Result<content::Model, StatusCode> {
    match Contents::find_by_id(content_id)
        .filter(Expr::cust("json_array_length(queue) > 0"))
        .one(database)
        .await
    {
        Ok(Some(content)) => Ok(content),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
