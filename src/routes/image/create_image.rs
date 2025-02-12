
use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::database::images;

#[derive(serde::Deserialize)]
pub struct ImageCreate {
    image_url: String
}

pub async fn create_image(Extension(database): Extension<DatabaseConnection>,Json(request_image): Json<ImageCreate>) {
    let new_images = images::ActiveModel {
        image_url: Set(Some(request_image.image_url)),
        created_at: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };
    let result = new_images.save(&database).await.unwrap();
    dbg!(result);
}