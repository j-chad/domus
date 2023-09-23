use crate::db::user::User;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use pasetors::claims::Claims;
use pasetors::errors::Error as ClaimError;
use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::public;
use pasetors::version4::V4;
use uuid::Uuid;

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

pub fn generate_auth_token(user: &User) -> Result<String, ClaimError> {
    let mut claims = Claims::new()?;
    claims.issuer("domus-api.jacksonc.dev")?;
    claims.subject(user.id.as_hyphenated().to_string().as_str())?;
    claims.audience("domus.jacksonc.dev")?;
    claims.token_identifier(Uuid::new_v4().to_string().as_str())?;

    // Generate the keys and sign the claims.
    let kp = AsymmetricKeyPair::<V4>::generate()?;
    public::sign(&kp.secret, &claims, None, None)
}
