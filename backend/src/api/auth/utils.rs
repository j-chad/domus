use crate::api::auth::models::AuthResponse;
use crate::api::error::ErrorType::UserAlreadyExists;
use crate::api::error::{APIError, APIErrorBuilder};
use crate::db::database::Connection;
use crate::db::refresh_token::{NewRefreshToken, RefreshToken};
use crate::db::schema::users;
use crate::db::user::{NewUser, User};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use pasetors::claims::Claims;
use pasetors::errors::Error as ClaimError;
use pasetors::keys::AsymmetricSecretKey;
use pasetors::public;
use pasetors::version4::V4;
use std::time::Duration;
use tracing::{error, warn};
use uuid::Uuid;

const TOKEN_EXPIRY_TIME: Duration = Duration::new(30 * 60, 0); // 30 minutes

pub fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let password_bytes = password.as_bytes();
    let password_hash = argon2::PasswordHash::new(hash)?;

    argon2.verify_password(password_bytes, &password_hash)
}

pub fn hash_password(password: &str) -> Result<String, APIError> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password_bytes, &salt).map_err(|err| {
        error!(error = %err, "failed to hash password");
        APIErrorBuilder::from_error(err).build()
    })?;

    Ok(password_hash.to_string())
}

pub async fn create_new_user(conn: &mut Connection, user: NewUser) -> Result<User, APIError> {
    diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(conn)
        .await
        .map_err(|e| {
            warn!(email = user.email, "failed to register new user: {:?}", e);
            APIErrorBuilder::new(UserAlreadyExists)
                .cause(e)
                .detail("If you already have an account, try logging in.")
                .with_field("email", user.email.into())
                .build()
        })
}

pub async fn generate_auth_tokens(
    conn: &mut Connection,
    user: &User,
    private_key: &str,
) -> Result<AuthResponse, APIError> {
    let refresh_token = generate_new_refresh_token(conn, user.id)
        .await?
        .id
        .to_string();
    let access_token = generate_auth_token(user, private_key).map_err(|e| {
        error!(error = %e, "failed to generate auth token");
        APIErrorBuilder::from_error(e).build()
    })?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
    })
}

pub async fn generate_new_refresh_token(
    conn: &mut Connection,
    user_id: Uuid,
) -> Result<RefreshToken, APIError> {
    diesel::insert_into(crate::db::schema::refresh_tokens::table)
        .values(&NewRefreshToken { user_id })
        .get_result::<RefreshToken>(conn)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to insert refresh token");
            APIErrorBuilder::from_error(e).build()
        })
}

fn generate_auth_token(user: &User, private_key: &str) -> Result<String, ClaimError> {
    let mut claims = Claims::new_expires_in(&TOKEN_EXPIRY_TIME)?;
    claims.issuer("domus-api.jacksonc.dev")?;
    claims.subject(user.id.as_hyphenated().to_string().as_str())?;
    claims.audience("domus.jacksonc.dev")?;
    claims.token_identifier(Uuid::new_v4().to_string().as_str())?;

    let key = AsymmetricSecretKey::<V4>::try_from(private_key)?;
    public::sign(&key, &claims, None, None)
}
