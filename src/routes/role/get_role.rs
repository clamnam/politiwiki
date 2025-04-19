use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Extension, Json, TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::roles::Entity as Roles;
use crate::database::users::{self, Entity as Users};

#[derive(serde::Serialize)]
pub struct ResponseRole {
    id: i32,
    title: f32,
}

pub async fn get_role(
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseRole>, StatusCode> {
    // Extract the token from the Authorization header
    let token = authorization.token();

    // Find user by token
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.to_string())))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extract role_id from user
    let role_id = user.role_id.ok_or(StatusCode::UNAUTHORIZED)?;

    // Query the Roles table using the role_id from the user
    let role = Roles::find_by_id(role_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(Json(ResponseRole {
        id: role.id,
        title: role.title,
    }))
}



