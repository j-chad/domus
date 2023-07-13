use crate::api::shared::errors::APIError;
use crate::db::models::user::User;
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Audience
    exp: i64,    // Expiration time (as UTC timestamp)
    iat: i64,    // Issued at (as UTC timestamp)
    iss: String, // Issuer
    sub: String, // Subject (whom token refers to)
    jti: String, // JWT ID

    // Custom claims
    home: String,     // House ID
    role: String,     // User role
    acl: Vec<String>, // User Level ACL
}

impl Claims {
    fn from_user(user: &User) -> Self {
        Self {
            aud: "domus.jacksonc.dev".to_string(),
            exp: Self::get_exp().timestamp(),
            iat: Utc::now().timestamp(),
            iss: "api.domus.jacksonc.dev".to_string(),
            sub: user.id.to_string(),
            jti: Uuid::new_v4().to_string(),
            role: "".to_string(),
            home: "".to_string(),
            acl: vec![],
        }
    }

    fn get_exp() -> DateTime<Utc> {
        Utc::now() + chrono::Duration::hours(1)
    }
}

pub fn generate_access_token(user: &User) -> Result<String, APIError> {
    let claims = Claims::from_user(user);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|_e| APIError::from_code(StatusCode::INTERNAL_SERVER_ERROR))
}

pub fn generate_refresh_token(user: &User) -> Result<String, APIError> {
    Ok("".to_string())
}
