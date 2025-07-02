pub use argon2::{
    password_hash::{
        Error as PasswordHashError, // Alias PasswordHash::Error to avoid conflict
        errors::B64Error,           // For base64 decoding errors if they occur
    },
    Error as Argon2Error, // Alias Argon2::Error to avoid conflict
};
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum PasswordError {
    HashingFailed(String),
    VerificationFailed(String),
    InvalidHashFormat(String),
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordError::HashingFailed(e) => write!(f, "Password hashing failed: {}", e),
            PasswordError::VerificationFailed(e) => write!(f, "Password verification failed: {}", e),
            PasswordError::InvalidHashFormat(e) => write!(f, "Invalid password hash format: {}", e),
        }
    }
}

impl Error for PasswordError {}
impl From<PasswordHashError> for PasswordError {
    fn from(err: PasswordHashError) -> Self {
        PasswordError::InvalidHashFormat(err.to_string())
    }
}
impl From<Argon2Error> for PasswordError {
    fn from(err: Argon2Error) -> Self {
        PasswordError::HashingFailed(err.to_string())
        }
}

impl From<B64Error> for PasswordError {
    fn from(err: B64Error) -> Self {
        PasswordError::InvalidHashFormat(format!("Base64 decoding error: {}", err))
    }
}

