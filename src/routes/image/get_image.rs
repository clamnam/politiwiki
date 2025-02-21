use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::database::images::Entity as Pages;
#[derive(serde::Serialize)]
pub struct ResponsePage {
    id: i32,
    image_url: String,
    created_at: String
    
}

// #[derive(serde::Deserialize)]
// pub struct GetPageQueryParams {
//     image_url: String,
// }

pub async fn get_single_image(Path(image_id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResponsePage>, StatusCode> {
    let image: Option<crate::database::images::Model> = Pages::find_by_id(image_id).one(&database).await.unwrap();
    if let Some(image) = image {
        Ok(Json(ResponsePage {
            id: image.id,
            image_url: image.image_url.to_string(),
            created_at: image.created_at.unwrap_or_default().to_string(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_image(
    Extension(database): Extension<DatabaseConnection>,
    // Query(query_params): Query<GetPageQueryParams>
    ) -> Result<Json<Vec<ResponsePage>>, StatusCode> {
    // dbg!(query_params.image_url.to_owned());

    // let mut image_url_filter = Condition::all();
    // if let Some(image_url) = query_params.image_url {
    //     image_url_filter = if !image_url.is_empty() {
    //         dbg!("image_url is not empty");
    //     image_url_filter.add(images::Column::ImageUrl.eq(image_url))
    //     } else {
    //         dbg!("image_url is empty");
    //         image_url_filter.add(images::Column::ImageUrl.is_null())

    //     }

    // }

    
    let images = Pages::find()
    // .filter(image_url_filter)
    .all(&database)
    .await
    .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|db_image| ResponsePage {
        id: db_image.id,
        image_url: db_image.image_url,
        created_at: db_image.created_at.unwrap_or_default().to_string(),
    })
    .collect();

Ok(Json(images))
}

pub async fn _get_single_fimage(){
    todo!()
}