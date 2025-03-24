use axum::http::StatusCode;
use axum::Json;
use axum::{extract::Path,Extension};
use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::pages::{self, Entity as Pages};
use sea_orm::prelude::DateTime;
use axum::response::IntoResponse;

#[derive(serde::Deserialize)]
pub struct RequestPage{
    pub title: String,
    pub created_at: Option<DateTime>,
    pub page_type: i32,
    pub history: Option<String>,
}
pub async fn atomic_update_page(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_page): Json<RequestPage>
) -> impl IntoResponse {
    let history = request_page.history.unwrap_or_default();
    let history = json::parse(&history).unwrap();
    let history = history.to_string();
    dbg!(&history);
    let update_page = pages::ActiveModel {
        id: Set(id),
        title: Set(request_page.title),
        created_at: Set(request_page.created_at),
        updated_at: Set(Some(Utc::now().naive_utc())),
        page_type: Set(Some(request_page.page_type)),
        history: Set(Some(sea_orm::JsonValue::String(history))),
    };

    match Pages::update(update_page)
        .filter(pages::Column::Id.eq(id))
        .exec(&database)
        .await
    {
        Ok(_) => StatusCode::OK, 
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}