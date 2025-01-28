
use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::database::users;

#[derive(serde::Deserialize)]
pub struct UserCreate {
    email: String,
    username: String,
    password: String,
}

pub async fn create_user(Extension(database): Extension<DatabaseConnection>,Json(request_user): Json<UserCreate>) {
    let new_users = users::ActiveModel {
        email : Set(request_user.email),
        username:Set(request_user.username),
        password:Set(request_user.password),
        created_at: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };
    let result =new_users.save(&database).await.unwrap();
    dbg!(result);
}