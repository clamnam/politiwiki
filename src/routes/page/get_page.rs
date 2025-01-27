use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::database::pages::Entity as Page;

#[derive(serde::Serialize)]
pub struct ResponsePage {
    id: i32,
    title: String,
    created_at: String,
    updated_at: String,
}

pub async fn get_single_page(Path(page_id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResponsePage>,StatusCode> {
    let page: Option<crate::database::pages::Model> = Page::find_by_id(page_id).one(&database).await.unwrap();
    if let Some(page) = page{
    Ok(Json(ResponsePage {
            id: page.id,
            title: page.title,
            created_at: page.created_at.unwrap().to_string(),
            updated_at: page.updated_at.unwrap_or_default().to_string(),
        }))
    }
    else{
        Err(StatusCode::NOT_FOUND)
    }

}
pub async fn get_all_pages(Extension(database): Extension<DatabaseConnection>) -> Result<Json<Vec<ResponsePage>>,StatusCode> {
    let pages:Vec<Model>  = Page::find()
    .all(&database)
    .await.map_err[|_error|StatusCode::INTERNAL_SERVER_ERROR]?
    .into_iter()
    .map(|db_task|ResponsePage{
        id:page.id,
        title:page.title,
        created_at:page.created_at.unwrap().to_string(),
        updated_at:page.updated_at.unwrap_or_default().to_string(),
    })

}