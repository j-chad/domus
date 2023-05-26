use super::model::RegisterNewUser;
use crate::db::connection::DbPool;
use crate::db::models::user::{create_user, NewUser};
use actix_web::{error, web};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

pub async fn register_user(
    pool: web::Data<DbPool>,
    user: RegisterNewUser,
) -> Result<(), error::Error> {
    let password_hash = hash_password(&user.password).unwrap();

    web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        create_user(
            &mut conn,
            NewUser {
                email: &user.email,
                first_name: &user.first_name,
                last_name: &user.last_name,
                password: &password_hash,
            },
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password_bytes, &salt)?;

    Ok(password_hash.to_string())
}
