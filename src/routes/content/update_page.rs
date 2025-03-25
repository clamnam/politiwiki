use axum::http::StatusCode;
use axum::Json;
use axum::{extract::Path,Extension};
use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::contents::{self, Entity as contents};
use sea_orm::prelude::DateTime;
use axum::response::IntoResponse;

#[derive(serde::Deserialize)]
pub struct Requestcontent{
    pub title: String,
    pub created_at: Option<DateTime>,
    pub content_type: i32,
    pub history: Option<String>,
}
pub async fn atomic_update_content(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_content): Json<Requestcontent>
) -> impl IntoResponse {
    let history = request_content.history.unwrap_or_default();
    let history = json::parse(&history).unwrap();
    let history = history.to_string();
    dbg!(&history);
    let update_content = contents::ActiveModel {
        id: Set(id),
        title: Set(request_content.title),
        created_at: Set(request_content.created_at),
        updated_at: Set(Some(Utc::now().naive_utc())),
        content_type: Set(Some(request_content.content_type)),
        history: Set(Some(sea_orm::JsonValue::String(history))),
    };

    match contents::update(update_content)
        .filter(contents::Column::Id.eq(id))
        .exec(&database)
        .await
    {
        Ok(_) => StatusCode::OK, 
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}