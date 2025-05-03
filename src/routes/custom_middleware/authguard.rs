use crate::{
    database::users::{self, Entity as Users},
    utils::jwt::is_valid,
};
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
pub async fn authguard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    // Extract Bearer token from request headers
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();
    
    // Get database connection from request extensions
    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Query database for user with matching token
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Verify JWT token is valid and not expired
    is_valid(&token)?;

    // Return 401 if user not found
    let Some(user) = user else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Add authenticated user to request extensions for downstream handlers
    request.extensions_mut().insert(user);
    
    // Continue to next middleware/handler
    Ok(next.run(request).await)
}
