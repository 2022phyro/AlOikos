use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::authentication::password;
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
    password_hash

}
pub fn verify_password(password: &str, password_hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    assert!(Argon2::default().verify_password(password, &parsed_hash).is_ok());
}