
use axum::{http::StatusCode, response::Response, Extension, Json};
use chrono::Utc;
use crate::database::users as Users;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::database::users;
#[derive(serde::Deserialize)]
pub struct RequestUser {
    email: Option<String>,
    username: String,
    password: String,
}
#[derive(serde::Serialize)]
pub struct ResponseUser {
    id: i32,
    email: String,
    username: String,
    token: String,
}


pub async fn register(Extension(database): Extension<DatabaseConnection>,Json(request_user): Json<RequestUser>) -> Result<Json<ResponseUser>,StatusCode> {
    let new_user = users::ActiveModel {
        email: Set(request_user.email.unwrap_or_default()),
        username:Set(request_user.username),
        password:Set(request_user.password),
        token: Set(Some("dslajkldsao9928913".to_owned())),
        created_at: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        id: new_user.id.unwrap(),
        email: new_user.email.unwrap(),
        username: new_user.username.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}
pub async fn login(Json(request_user): Json<RequestUser>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResponseUser>,StatusCode> {
    let db_user = users::Entity::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(db_user)=db_user {
        let new_token = "1231412412312".to_owned();
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));
        let saved_user = user.save(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;


            Ok (Json(ResponseUser {
            id: saved_user.id.unwrap(),
            email: saved_user.email.unwrap(),
            username: saved_user.username.unwrap(),
            token: saved_user.token.unwrap().unwrap()

        }))
    }else{
        return Err(StatusCode::UNAUTHORIZED);
    }
            }