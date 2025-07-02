use core::panic;

use totp_rs::{TOTP, Secret, Algorithm};
use crate::config::CONFIG;
use base32::{Alphabet, encode};
use rand::Rng;
use hex;

#[derive(Debug, Clone)]
pub struct Otp {
    pub otp_type: String,
    pub secret: String,
    pub generator: TOTP,
    pub user_email: String,
}

impl Otp {
    /// Create a new OTP instance with a provided secret
    pub fn new(raw_secret: &str, user_email: &str, possible_otp_type: Option<String>) -> Result<Self, String> {
        if raw_secret.is_empty() {
            return Err("Secret cannot be empty".to_string());
        }
        
        if user_email.is_empty() {
            return Err("User email cannot be empty".to_string());
        }

        // Convert secret to base32
        let base32_secret = encode(Alphabet::RFC4648 { padding: false }, raw_secret.as_bytes());
        let secret = Secret::Encoded(base32_secret.clone());

        let otp_type = possible_otp_type.unwrap_or_else(|| "EMAIL".to_string());
        let expiry: u64 = if otp_type.to_uppercase() == "EMAIL" {
            CONFIG.otp_expiry.parse::<u64>().unwrap_or(30)
        } else {
            30
        };

        let generator = TOTP::new(
            Algorithm::SHA1,
            CONFIG.otp_digit_length,
            CONFIG.otp_skew,
            expiry,
            secret.to_bytes().unwrap(),
            Some(CONFIG.otp_issuer.clone()),
            user_email.to_string(),
        ).map_err(|e| format!("Failed to create TOTP generator: {}", e))?;

        Ok(Otp {
            otp_type,
            secret: base32_secret,
            generator,
            user_email: user_email.to_string(),
        })
    }

    /// Create a new OTP instance with a randomly generated secret
    pub fn new_with_random_secret(user_email: &str, possible_otp_type: Option<String>) -> Result<Self, String> {
        let random_secret = Self::generate_random_secret();
        Self::new(&random_secret, user_email, possible_otp_type)
    }

    /// Generate a random base32 secret
    pub fn generate_random_secret() -> String {
        let mut rng = rand::thread_rng();
        let secret_bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
        encode(Alphabet::RFC4648 { padding: false }, &secret_bytes)
    }

    /// Generate a new OTP code
    pub fn generate_otp(&self) -> Result<String, String> {
        self.generator
            .generate_current()
            .map_err(|e| format!("Failed to generate OTP: {}", e))
    }

    /// Verify an OTP code
    pub fn verify_otp(&self, otp: &str) -> Result<bool, String> {
        if otp.is_empty() {
            return Err("OTP cannot be empty".to_string());
        }

        self.generator
            .check_current(otp)
            .map_err(|e| format!("Failed to verify OTP: {}", e))
    }

    /// Generate QR code as base64 string
    pub fn qr_code_base64(&self) -> Result<String, String> {
        self.generator
            .get_qr_base64()
            .map_err(|e| format!("Failed to generate QR code: {}", e))
    }

    /// Get the OTP URL for manual entry into authenticator apps
    pub fn get_url(&self) -> String {
        if self.otp_type.to_uppercase() == "EMAIL" {
            panic!("Email OTPs do not have a URL");
        }
        self.generator.get_url()
    }

    /// Get the secret in base32 format
    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    /// Get the OTP type
    pub fn get_otp_type(&self) -> &str {
        &self.otp_type
    }

    /// Get the user email
    pub fn get_user_email(&self) -> &str {
        &self.user_email
    }
    /// Generate a random 128-bit (16 bytes) secret as a raw string (not encoded)
    pub fn generate_random_secret_raw() -> String {
        let mut rng = rand::thread_rng();
        let secret_bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        // Convert bytes to a hex string for readability, or use as needed
        hex::encode(secret_bytes)
    }
}
