use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct RegisterNewUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 1))]
    first_name: String,
    #[validate(length(min = 1))]
    last_name: String,
    #[validate(length(min = 8, max = 64))]
    password: String,
}
