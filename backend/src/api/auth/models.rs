use crate::api::auth::utils::hash_password;
use crate::db::models::{NewUser, User};
use crate::utils::friendly_id::{ItemIdType, ToFriendlyId};
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::error;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterNewUserRequest {
    #[validate(email)]
    #[schema(example = "john.smith@example.com", format = "email")]
    pub email: String,

    #[validate(length(min = 1))]
    #[schema(example = "John", min_length = 1)]
    pub first_name: String,

    #[validate(length(min = 1))]
    #[schema(example = "Smith", min_length = 1)]
    pub last_name: String,

    #[validate(length(min = 8))]
    #[schema(example = "Password123", min_length = 8, format = "password")]
    pub password: String,
}

impl fmt::Debug for RegisterNewUserRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegisterNewUserRequest")
            .field("email", &self.email)
            .field("first_name", &self.first_name)
            .field("last_name", &self.last_name)
            .field("password", &"********")
            .finish()
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginUserRequest {
    #[validate(email)]
    #[schema(example = "john.smith@example.com", format = "email")]
    pub email: String,

    #[validate(length(min = 8))]
    #[schema(example = "Password123", min_length = 8, format = "password")]
    pub password: String,
}

impl fmt::Debug for LoginUserRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoginUserRequest")
            .field("email", &self.email)
            .field("password", &"********")
            .finish()
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[schema()]
    pub refresh_token: String,
}

impl fmt::Debug for RefreshTokenRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefreshTokenRequest")
            .field("refresh_token", &"********")
            .finish()
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_friendly_id(ItemIdType::User),
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}
