use core::panic;

use totp_rs::{TOTP, Secret, Algorithm};
use crate::{authentication::otp, config::CONFIG};
use base32::{Alphabet, encode};
use rand::Rng;
use hex;
use hkdf::Hkdf;
use sha2::Sha256;

#[derive(Debug, Clone, StrChoice)]
#[strchoice(case="snake")]
pub enum OTPAction {
    VerifyAccount,
    ChangePassword,
    MakeTransaction,
    TwoFaLogin,
}

#[derive(Debug, Clone, StrChoice)]
#[strchoice(case="snake")]
pub enum OtpType {
    EMAIL,
    AUTHENTICATOR
}


#[derive(Debug, Clone)]
pub struct Otp {
    pub otp_type: OtpType,
    pub secret: String,
    pub generator: TOTP,
    pub user_email: String,
}

impl Otp {

    /// Derive a unique key for the user and action
    pub fn derive_key(user_email: &str, action: OTPAction) -> Vec<u8> {
        let hk = Hkdf::<Sha256>::new(None, CONFIG.otp_encryption_key.as_bytes());
        let mut okm = vec![0u8; 20]; // Length for SHA1
        hk.expand(action.to_str().as_bytes(), &mut okm).expect("HKDF expand failed");
        hk.expand(user_email.as_bytes(), &mut okm).expect("User email didn't return, explicit expand failed");
        okm
    }

    /// Create a new OTP instance
    pub fn new(user_email: &str, action: OTPAction, otp_type: OtpType) -> Result<Self, String> {
        let derived_key_bytes = Self::derive_key(user_email, action);
        let base32_secret = encode(Alphabet::RFC4648 { padding: false }, derived_key_bytes);
        let secret = Secret::Encoded(base32_secret.clone());
        let expiry =  {
            match otp_type {
                OtpType::EMAIL => CONFIG.otp_expiry.parse::<u64>().unwrap_or(300),
                OtpType::AUTHENTICATOR => 30,
            }
        };
        let generator = TOTP::new(
            Algorithm::SHA256,
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
        match self.otp_type {
            OtpType::EMAIL => panic!("Email OTPs do not have a URL"),
            OtpType::AUTHENTICATOR => return self.generator.get_url(),
        }
    }

    /// Get the secret in base32 format
    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    /// Get the OTP type
    pub fn get_otp_type(&self) -> OtpType {
        self.otp_type.clone()
    }

    /// Get the user email
    pub fn get_user_email(&self) -> &str {
        &self.user_email
    }
}
