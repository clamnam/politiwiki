use axum::http::StatusCode;
use axum::Json;
use axum::{extract::Path,Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::images::{self, Entity as Images};
use sea_orm::prelude::DateTime;
use axum::response::IntoResponse;

#[derive(serde::Deserialize)]
pub struct RequestImage{
    pub image_url: String,
    // pub content_id: Option<i32>,
    pub created_at: Option<DateTime>,
}
pub async fn atomic_update_image(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_image): Json<RequestImage>
) -> impl IntoResponse {
    let update_image = images::ActiveModel {
        id: Set(id),
        image_url: Set(Some(request_image.image_url)),
        created_at: Set(request_image.created_at),
    };

    match Images::update(update_image)
        .filter(images::Column::Id.eq(id))
        .exec(&database)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}