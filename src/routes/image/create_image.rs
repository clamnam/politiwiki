use axum::{
    extract::{Extension, Multipart},
    response::IntoResponse,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use crate::database::images;

// #[derive(serde::Deserialize)]
// pub struct ImageCreate {
//     image_data: Vec<u8>,

// }

pub async fn create_image(
    Extension(database): Extension<DatabaseConnection>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // dbg!("create_image");
    // Iterate over multipart fields
    while let Some(field) = multipart.next_field().await.unwrap() {
        // We assume the file field is named "file"
        if let Some(field_name) = field.name() {
            if field_name == "file" {
                // Check the MIME type to ensure it's an image
                if let Some(content_type) = field.content_type() {
                    if !content_type.to_string().starts_with("image/") {
                        return "Only image files are allowed".into_response();
                    }
                }
                // Get the file bytes and convert to Vec<u8>
                let bytes = field.bytes().await.unwrap();
                let image_vec: Vec<u8> = bytes.to_vec();
                
                // Create a new DB entry
                let new_image = images::ActiveModel {
                    image_data: Set(image_vec),
                    created_at: Set(Some(Utc::now().naive_utc())),
                    ..Default::default()
                };
                let result = new_image.save(&database).await.unwrap();
                return format!("Image uploaded with id {:?}", result).into_response();
            }
        }
    }
    "No file found in the request".into_response()
}

