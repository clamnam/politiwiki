
use crate::utils::jwt::create_jwt;
use axum::{headers::{authorization::Bearer, Authorization}, http::StatusCode, Extension, Json, TypedHeader};
use chrono::Utc;

use crate::database::{roles, users::Entity as Users};

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::database::users;
#[derive(serde::Deserialize,Debug)]
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
    let jwt = create_jwt()?;
    let new_role = roles::ActiveModel {
        title: Set(Some(5)),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user = users::ActiveModel {
        email: Set(request_user.email.unwrap_or_default()),
        username:Set(request_user.username),
        password:Set(hash_password(request_user.password)?),
        token: Set(Some(jwt)),
        created_at: Set(Some(Utc::now().naive_utc())),
        role_id: Set(Some(new_role.id.unwrap())),
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
        println!("Request user: {:?}", &request_user); let db_user = users::Entity::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(db_user)=db_user {
        if !verify_password(request_user.password, &db_user.password)?{
            return Err(StatusCode::UNAUTHORIZED);
        }
        let new_token = create_jwt()?;
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));
        let saved_user = user.save(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;


            Ok (Json(ResponseUser {
            id: saved_user.id.unwrap(),
            email: saved_user.email.unwrap(),
            username: saved_user.username.unwrap(),
            token: saved_user.token.unwrap().unwrap()}))
    }else{
        return Err(StatusCode::UNAUTHORIZED);
    }
}

pub async fn logout(authorization: TypedHeader<Authorization<Bearer>>, Extension(database): Extension<DatabaseConnection>)-> Result<(),StatusCode> {
    let token = authorization.token();
    dbg!(token);
    let mut user = if let Some(user)= Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_error| {
            eprintln!("{}", _error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
{
    user.into_active_model()
}else{
    return Err(StatusCode::UNAUTHORIZED);
};
user.token=Set(None); 

user.save(&database).await.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
Ok(())
}

fn hash_password (password: String)->Result<String,StatusCode>{
    bcrypt::hash(password,11)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
fn verify_password(password: String, hash:&str) ->Result<bool,StatusCode>{
    bcrypt::verify(password,hash)
    .map_err(|_errpr|StatusCode::UNAUTHORIZED)
}