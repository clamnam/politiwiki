use crate::utils::jwt::create_jwt;
use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Extension, Json, TypedHeader,
};
use chrono::{NaiveDateTime, Utc};

use crate::database::{roles, users::Entity as Users};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::database::users;
#[derive(serde::Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}
#[derive(serde::Deserialize, Debug)]
pub struct RequestRegisterUser {
    email: String,
    username: String,
    password: String,
}
#[derive(serde::Serialize)]
pub struct ResponseUser {
    id: i32,
    email: String,
    username: String,
    token: String,
    role_id: i32,
    created_at: NaiveDateTime,
}

pub async fn register(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestRegisterUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    // Jwt helper function used to make a unique jwt
    let jwt = create_jwt()?;
    // set default auth level
    let new_role = roles::ActiveModel {
        title: Set(0.5),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // create new instance of user using request info and generated info  then saves to database
    let new_user = users::ActiveModel {
        email: Set(request_user.email),
        username: Set(request_user.username),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(jwt)),
        created_at: Set(Some(Utc::now().naive_utc())),
        role_id: Set(Some(new_role.id.unwrap())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //set response content
    Ok(Json(ResponseUser {
        id: new_user.id.unwrap(),
        email: new_user.email.unwrap(),
        username: new_user.username.unwrap(),
        token: new_user.token.unwrap().unwrap(),
        role_id: new_user.role_id.unwrap().unwrap_or_default(),
        created_at: new_user.created_at.unwrap().unwrap(),
    }))
}

pub async fn login(
    Json(request_user): Json<RequestUser>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    // check database for user with the same username
    let db_user = users::Entity::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?; 

    // Handle case where user is not found
    let db_user = match db_user {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED), // User not found = unauthorized
    };
    
    // Verify password
    if !verify_password(request_user.password, &db_user.password)? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let new_token = create_jwt()?;
    let mut user = db_user.into_active_model();
    user.token = Set(Some(new_token));
    let saved_user = user
        .save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        id: saved_user.id.unwrap(),
        email: saved_user.email.unwrap(),
        username: saved_user.username.unwrap(),
        token: saved_user.token.unwrap().unwrap(),
        role_id: saved_user.role_id.unwrap().unwrap_or_default(),
        created_at: saved_user.created_at.unwrap().expect("created_at missing"),
    }))
}

pub async fn logout(
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    //retrieve token
    let token = authorization.token();
    // validates user
    let mut user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_error| {
            eprintln!("{}", _error);
            StatusCode::INTERNAL_SERVER_ERROR
        })? {
        user.into_active_model()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    //erase token from database
    user.token = Set(None);

    user.save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 11).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_errpr| StatusCode::UNAUTHORIZED)
}
