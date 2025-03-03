use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::database::images::Entity as Pages;
#[derive(serde::Serialize)]
pub struct ResponsePage {
    id: i32,
    image_data: Vec<u8>,
    created_at: String
    
}

// #[derive(serde::Deserialize)]
// pub struct GetPageQueryParams {
//     image_data: String,
// }
// TODO CHANGE TO HANDLE RETURNING IMAGES 
pub async fn get_single_image(Path(image_id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResponsePage>, StatusCode> {
    let image: Option<crate::database::images::Model> = Pages::find_by_id(image_id).one(&database).await.unwrap();
    if let Some(image) = image {
        Ok(Json(ResponsePage {
            id: image.id,
            image_data: image.image_data,
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
    // dbg!(query_params.image_data.to_owned());

    // let mut image_data_filter = Condition::all();
    // if let Some(image_data) = query_params.image_data {
    //     image_data_filter = if !image_data.is_empty() {
    //         dbg!("image_data is not empty");
    //     image_data_filter.add(images::Column::ImageUrl.eq(image_data))
    //     } else {
    //         dbg!("image_data is empty");
    //         image_data_filter.add(images::Column::ImageUrl.is_null())

    //     }

    // }

    
    let images = Pages::find()
    // .filter(image_data_filter)
    .all(&database)
    .await
    .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|db_image| ResponsePage {
        id: db_image.id,
        image_data: db_image.image_data,
        created_at: db_image.created_at.unwrap_or_default().to_string(),
    })
    .collect();

Ok(Json(images))
}

pub async fn _get_single_fimage(){
    todo!()
}