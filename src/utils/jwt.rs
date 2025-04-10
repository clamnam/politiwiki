use axum::http::StatusCode;
use chrono::{Duration, Utc};
use std::env;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    exp: usize,
    iat: usize
    // TODO: add subject to uniquely identify the user
}

pub fn create_jwt() -> Result<String,StatusCode>{ 
    
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::hours(24);
    now+=expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims{
        exp: exp,
        iat: iat
    };
    let secret = env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let key = EncodingKey::from_secret(secret.as_bytes());
    let token = encode(&Header::default(),&claim,&key);
    token.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}
pub fn is_valid(token: &str) -> Result<bool ,StatusCode>{
    let secret = env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(
        |error| match error.kind(){
            jsonwebtoken::errors::ErrorKind::ExpiredSignature=> StatusCode::UNAUTHORIZED,
            _=> StatusCode::INTERNAL_SERVER_ERROR
        }
    )?;
        
    Ok(true)
}