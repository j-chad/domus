use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password_bytes, &salt)?;

    Ok(password_hash.to_string())
}
