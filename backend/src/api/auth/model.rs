use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterNewUser {
    #[validate(email)]
    #[schema(example = "john.smith@example.com")]
    pub email: String,

    #[validate(length(min = 1))]
    #[schema(example = "John", min_length = 1)]
    pub first_name: String,

    #[validate(length(min = 1))]
    #[schema(example = "Smith", min_length = 1)]
    pub last_name: String,

    #[validate(length(min = 8, max = 64))]
    #[schema(example = "Password123", min_length = 8, max_length = 64)]
    pub password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginUser {
    #[validate(email)]
    #[schema(example = "john.smith@example.com")]
    pub email: String,

    #[validate(length(min = 8, max = 64))]
    #[schema(example = "Password123", min_length = 8, max_length = 64)]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}
