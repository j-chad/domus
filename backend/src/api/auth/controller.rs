use actix_web::*;

#[get("/register")]
pub async fn register() -> String {
    "Hello there!".to_string()
}
