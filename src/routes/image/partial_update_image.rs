use axum::http::StatusCode;
use axum::Json;
use axum::{extract::Path,Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::images::{self, Entity as Pages};
use sea_orm::prelude::DateTime;
use axum::response::IntoResponse;

#[derive(serde::Deserialize)]
pub struct RequestPage{
    pub image_url: String,
    pub created_at: Option<DateTime>,
}
pub async fn partial_update_image(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_image): Json<RequestPage>
) -> impl IntoResponse {
    let update_image = images::ActiveModel {
        id: Set(id),
        image_url: Set(request_image.image_url),
        created_at: Set(request_image.created_at),
    };

    match Pages::update(update_image)
        .filter(images::Column::Id.eq(id))
        .exec(&database)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}