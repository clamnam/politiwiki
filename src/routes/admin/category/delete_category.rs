use axum::http::StatusCode;
use axum::{
    headers::{authorization::Bearer, Authorization},
    Extension, Json, TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

use crate::database::users::{self, Entity as Users, Model};
use crate::database::{categories, roles};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RequestCategory {
    id: i32,
}
#[derive(serde::Serialize)]
pub struct CategoryCreateResponse {
    id: i32,
    name: String,
}

// #[debug_handler]
pub async fn delete_category(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_category): Json<RequestCategory>,
    authorization: TypedHeader<Authorization<Bearer>>,
) -> Result<Json<CategoryCreateResponse>, StatusCode> {
    let token = authorization.token();

    let user = match find_user(token, &database).await {
        Ok(user) => user,
        Err(_status) => return Err(StatusCode::UNAUTHORIZED),
    };
    // 3. Get the user's role
    let role = match find_role(&database, user).await {
        Ok(role) => role,
        Err(_status) => return Err(StatusCode::UNAUTHORIZED),
    };

    // 4. Check if role is at least 5, admin threshold
    if role.title < 0.5 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let category = match categories::Entity::find_by_id(request_category.id)
        .one(&database)
        .await
    {
        Ok(Some(category)) => category,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    categories::Entity::delete_by_id(request_category.id)
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CategoryCreateResponse {
        id: category.id,
        name: category.name,
    }))
}

async fn find_user(token: &str, database: &DatabaseConnection) -> Result<users::Model, StatusCode> {
    match Users::find()
        .filter(crate::database::users::Column::Token.eq(Some(token)))
        .one(database)
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn find_role(database: &DatabaseConnection, user: Model) -> Result<roles::Model, StatusCode> {
    let role = match user
        .find_related(crate::database::roles::Entity)
        .one(database)
        .await
    {
        Ok(Some(role)) => Ok(role),
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    role
}
