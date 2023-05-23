use crate::api::auth::model::RegisterNewUser;
use actix_web::*;
use log;
use validator::Validate;

#[post("/register")]
pub async fn register(user: web::Json<RegisterNewUser>) -> HttpResponse {
    log::trace!("Registering new user: {:?}", user);

    let val_result = user.validate();
    if let Err(e) = val_result {
        return HttpResponse::BadRequest().json(e);
    }

    println!("Registering new user: {:?}", user);
    HttpResponse::NoContent().finish()
}
