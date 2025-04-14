use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::database::content::Entity as Contents;
use crate::database::sea_orm_active_enums;

#[derive(serde::Deserialize,serde::Serialize)]
pub enum StatusValues {
    Pending,
    Approved,
    Rejected,
    Published,
}

impl From<sea_orm_active_enums::Status> for StatusValues {
    fn from(status: sea_orm_active_enums::Status) -> Self {
        match status {
            sea_orm_active_enums::Status::Pending   => StatusValues::Pending,
            sea_orm_active_enums::Status::Approved  => StatusValues::Approved,
            sea_orm_active_enums::Status::Rejected  => StatusValues::Rejected,
            sea_orm_active_enums::Status::Published => StatusValues::Published,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ResponseContent {
    id: i32,

    title: String,
    content_type: i32,
    content_body: String,
    images_id: Option<i32>,
    created_by_id: i32,
    modified_by_id: Option<i32>,
    order_id: i32,
    page_id: i32,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    status: Option<StatusValues>,  // Changed from sea_orm_active_enums::Status to StatusValues

    is_hidden: Option<bool>,
    is_deleted: Option<bool>,
    queue: Option<String>,
    history: Option<String>,

}

pub async fn get_single_content(Path(content_id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResponseContent>, StatusCode> {
    let content_result: Option<crate::database::content::Model> = Contents::find_by_id(content_id).one(&database).await.unwrap();
    
    if let Some(content) = content_result {
        // Now access queue from the unwrapped content
        if let Some(queue_data) = &content.queue {
            let queue_parsed = json::parse(&queue_data.to_string()).unwrap();
            let queue_string = queue_parsed.to_string();
            dbg!(&queue_string);
        }
        
        Ok(axum::Json(ResponseContent {
            id: content.id,
            title: content.title,
            content_type: content.content_type,
            content_body: content.content_body,
            images_id: content.images_id,
            created_by_id: content.created_by_id,
            modified_by_id: content.modified_by_id,
            created_at: Some(content.created_at),
            updated_at: content.updated_at,
            status: Some(content.status.into()),  // Wrap the non-optional Status in Some
            order_id: content.order_id.unwrap_or_default(),
            queue: content.queue.map(|q| q.to_string()),
            history: content.history.map(|h| h.to_string()),

            page_id: content.page_id,
            is_hidden: Some(content.is_hidden),
            is_deleted: Some(content.is_deleted),
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
            created_at: Some(content.created_at),
            updated_at: content.updated_at,
            modified_by_id: content.modified_by_id,
            page_id: content.page_id,
            status: Some(content.status.into()),  // Wrap the non-optional Status in Some
            order_id: content.order_id.unwrap_or_default(),
            is_hidden: Some(content.is_hidden),
            queue: content.queue.map(|q| q.to_string()),
            history: content.history.map(|h| h.to_string()),
            is_deleted: Some(content.is_deleted),
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
        title: db_content.title.to_string(),
        content_type: db_content.content_type,
        content_body: db_content.content_body,
        images_id: Some(db_content.images_id.unwrap_or_default()),
        created_by_id: db_content.created_by_id,
        modified_by_id: Some(db_content.modified_by_id.unwrap_or_default()),
        status: Some(db_content.status.into()),  // Wrap the non-optional Status in Some
        order_id: db_content.order_id.unwrap_or_default(),
        page_id: db_content.page_id,
        queue: db_content.queue.map(|q| q.to_string()),
        history: db_content.history.map(|h| h.to_string()),

        is_hidden: Some(db_content.is_hidden),
        is_deleted: Some(db_content.is_deleted),
        created_at: Some(db_content.created_at),
        updated_at: Some(db_content.updated_at.unwrap_or_default()),
    })
    .collect();

Ok(Json(contents))
}


