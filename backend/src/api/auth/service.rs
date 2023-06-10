use super::model::RegisterNewUser;
use crate::api::auth::model::{AuthResponse, LoginUser};
use crate::api::auth::utils::tokens::generate_access_token;
use crate::api::shared::errors::APIError;
use crate::db::connection::{get_connection, DbPool};
use crate::db::models::user::{create_user, find_user_by_email, NewUser};
use actix_web::http::StatusCode;
use actix_web::{error, web};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

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

pub async fn login_user(
    pool: web::Data<DbPool>,
    login: LoginUser,
) -> Result<AuthResponse, APIError> {
    let user = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = get_connection(pool)?;

        find_user_by_email(&mut conn, &login.email)
            .map_err(|_e| APIError::from_code(StatusCode::INTERNAL_SERVER_ERROR))
    })
    .await
    .map_err(|_e| APIError::from_code(StatusCode::INTERNAL_SERVER_ERROR))?;

    let password_matches: bool = match user {
        Ok(ref user) => verify_password(&login.password, &user.password).is_ok(),
        _ => {
            // Prevent timing side channel attacks by always taking the same amount of time to verify a password.
            let _ = verify_password("", "").is_ok();
            false
        }
    };

    if !password_matches {
        return Err(APIError {
            code: StatusCode::UNAUTHORIZED,
            message: Some("Invalid credentials".to_string()),
        });
    }

    let user = user?;
    let access_token = generate_access_token(&user)?;

    Ok(AuthResponse {
        access_token,
        refresh_token: "".to_string(),
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let password_bytes = password.as_bytes();
    let password_hash = argon2::PasswordHash::new(hash)?;

    argon2.verify_password(password_bytes, &password_hash)
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password_bytes, &salt)?;

    Ok(password_hash.to_string())
}
