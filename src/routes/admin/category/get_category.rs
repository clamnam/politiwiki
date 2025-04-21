use axum::http::StatusCode;
use axum::{debug_handler, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::database::categories;

#[derive(serde::Serialize)]
pub struct ResponseCategory {
    id: i32,
    name: String,
}



// pub async fn get_single_category(Path(page_id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResponseCategory>, StatusCode> {
//     let category: Option<crate::database::pages::Model> = Categories::find_by_id(page_id).one(&database).await.unwrap();
//     if let Some(category) = category {
//         Ok(Json(ResponseCategory {
//             id: category.id,
//             title: category.title,
//             created_at: category.created_at.unwrap_or_default().to_string(),
//             updated_at: category.updated_at.unwrap_or_default().to_string(),
//             category: category.category.unwrap_or_default(),
//         }))
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }
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
