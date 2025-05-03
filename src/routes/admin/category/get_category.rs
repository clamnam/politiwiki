use axum::http::StatusCode;
use axum::{debug_handler, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::database::categories;

#[derive(serde::Serialize)]
pub struct ResponseCategory {
    id: i32,
    name: String,
}

#[debug_handler]
pub async fn get_all_categories(
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseCategory>>, StatusCode> {
    dbg!("here");
    let categories = categories::Entity::find()
        .all(&database)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_page| ResponseCategory {
            id: db_page.id,
            name: db_page.name,
        })
        .collect();

    Ok(Json(categories))
}
