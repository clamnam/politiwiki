use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set};
use axum::{headers::{authorization::Bearer, Authorization}, Extension, Json, TypedHeader};
use axum::http::StatusCode;


use crate::database::{categories, roles};
use crate::database::users::{self, Entity as Users, Model};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RequestCategory {
    name: String,
}
#[derive(serde::Serialize)]
pub struct CategoryCreateResponse {
    id: i32,
    name: String,
}
pub async fn create_category(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_category): Json<RequestCategory>,
    authorization: TypedHeader<Authorization<Bearer>>,
)-> Result<Json<CategoryCreateResponse>, StatusCode>  {


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
    let new_category = categories::ActiveModel{
        name: Set(request_category.name),
        //id autoincrements
        ..Default::default() 
    };
    match new_category.save(&database).await {
        Ok(model) => {
            // Return the ID of the newly created categor
            Ok(Json(CategoryCreateResponse { id: model.id.unwrap(),name:model.name.unwrap() }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
    
}
// TODO abstract find_blank to utilities folder

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