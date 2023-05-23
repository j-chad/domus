use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterNewUser {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}
