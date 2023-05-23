use crate::api::auth::model::RegisterNewUser;
use actix_web::*;
use crate::db::models::user::create_user;

#[get("/register")]
pub async fn register(user: web::Json<RegisterNewUser>) -> impl Responder {
    create_user(user);

    HttpResponse::NoContent()
}
