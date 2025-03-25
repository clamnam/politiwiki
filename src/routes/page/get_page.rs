use axum::{extract::{Path, Query}, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use crate::database::pages::{self, Entity as Pages};
#[derive(serde::Serialize)]
pub struct ResponsePage {
    id: i32,
    title: String,
    created_at: String,
    updated_at: String,
    page_type: i32,
    
}

#[derive(serde::Deserialize)]
pub struct GetPageQueryParams {
    title: Option<String>,
}

pub async fn get_single_page(Path(page_id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResponsePage>, StatusCode> {
    let page: Option<crate::database::pages::Model> = Pages::find_by_id(page_id).one(&database).await.unwrap();
    if let Some(page) = page {
        Ok(Json(ResponsePage {
            id: page.id,
            title: page.title,
            created_at: page.created_at.unwrap_or_default().to_string(),
            updated_at: page.updated_at.unwrap_or_default().to_string(),
            page_type: page.page_type.unwrap_or_default(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_page(Extension(database): Extension<DatabaseConnection>, Query(query_params): Query<GetPageQueryParams>) -> Result<Json<Vec<ResponsePage>>, StatusCode> {
    dbg!(query_params.title.to_owned());

    let mut title_filter = Condition::all();
    if let Some(title) = query_params.title {
        title_filter = if !title.is_empty() {
            // dbg!("title is not empty");
        title_filter.add(pages::Column::Title.eq(title))
        } else {
            // dbg!("title is empty");
            title_filter.add(pages::Column::Title.is_null())

        }

    }

    
    let pages = Pages::find()
    .filter(title_filter)
    .all(&database)
    .await
    .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|db_page| ResponsePage {
        id: db_page.id,
        title: db_page.title,
        created_at: db_page.created_at.unwrap_or_default().to_string(),
        updated_at: db_page.updated_at.unwrap_or_default().to_string(),
        page_type: db_page.page_type.unwrap_or_default(),
    })
    .collect();

Ok(Json(pages))
}