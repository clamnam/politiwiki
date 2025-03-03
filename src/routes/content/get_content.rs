use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::database::content::Entity as Contents;
#[derive(serde::Serialize)]
pub struct ResponseContent {
    id: i32,
    title: Option<String>,
    content_type: Option<i32>,
    content_body: Option<String>,
    images_id: Option<i32>,
    created_by_id: Option<i32>,
    modified_by_id: Option<i32>,
    status: Option<i32>,
    page_id: Option<i32>,
    order_id: Option<i32>,
    is_hidden: Option<bool>,
    is_deleted: Option<bool>,



}


pub async fn get_single_content(Path(content_id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResponseContent>, StatusCode> {
    let content: Option<crate::database::content::Model> = Contents::find_by_id(content_id).one(&database).await.unwrap();
    if let Some(content) = content {

        Ok(Json(ResponseContent {
            id: content.id,
            title: content.title,
            content_type: content.content_type,
            content_body: content.content_body,
            images_id: content.images_id,
            created_by_id: content.created_by_id,
            modified_by_id: content.modified_by_id,
            status: content.status,
            order_id: content.order_id,
            page_id: content.page_id,

            is_hidden: content.is_hidden,
            is_deleted: content.is_deleted,

            
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
pub async fn get_content_by_page(Path(page_id): Path<i32>, Extension(database): Extension<DatabaseConnection>)-> Result<Json<Vec<ResponseContent>>, StatusCode>{
    let content = Contents::find()
        .filter(crate::database::content::Column::PageId.eq(page_id))
        .all(&database)
        .await
        .unwrap();
    if !content.is_empty() {
        Ok(Json(content.into_iter().map(|content| ResponseContent {
            id: content.id,
            title: content.title,
            content_type: content.content_type,
            content_body: content.content_body,
            images_id: content.images_id,
            created_by_id: content.created_by_id,
            modified_by_id: content.modified_by_id,
            page_id: content.page_id,
            status: content.status,
            order_id: content.order_id,
            is_hidden: content.is_hidden,
            is_deleted: content.is_deleted,
        }).collect()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
pub async fn get_all_content(
    Extension(database): Extension<DatabaseConnection>,
    // Query(query_params): Query<GetContentQueryParams>
    ) -> Result<Json<Vec<ResponseContent>>, StatusCode> {
    // dbg!(query_params.content_url.to_owned());

    // let mut content_url_filter = Condition::all();
    // if let Some(content_url) = query_params.content_url {
    //     content_url_filter = if !content_url.is_empty() {
    //         dbg!("content_url is not empty");
    //     content_url_filter.add(contents::Column::contentUrl.eq(content_url))
    //     } else {
    //         dbg!("content_url is empty");
    //         content_url_filter.add(contents::Column::contentUrl.is_null())

    //     }

    // }

    
    let contents = Contents::find()
    // .filter(content_url_filter)
    .all(&database)
    .await
    .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|db_content| ResponseContent {
        id: db_content.id,
        title: Some(db_content.title.unwrap_or_default().to_string()),
        content_type: Some(db_content.content_type.unwrap_or_default()),
        content_body: Some(db_content.content_body.unwrap_or_default()),
        images_id: Some(db_content.images_id.unwrap_or_default()),
        created_by_id: Some(db_content.created_by_id.unwrap_or_default()),
        modified_by_id: Some(db_content.modified_by_id.unwrap_or_default()),
        status: Some(db_content.status.unwrap_or_default()),
        order_id: Some(db_content.order_id.unwrap_or_default()),
        page_id: Some(db_content.page_id.unwrap_or_default()),
        is_hidden: Some(db_content.is_hidden.unwrap_or_default()),
        is_deleted: Some(db_content.is_deleted.unwrap_or_default()),
    })
    .collect();

Ok(Json(contents))
}

pub async fn _get_single_fcontent(){
    todo!()
}
