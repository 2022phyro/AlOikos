use core::panic;
use std::fmt::Display;

use totp_rs::{TOTP, Secret, Algorithm};
use crate::config::CONFIG;
use base32::{Alphabet, encode};
use hkdf::Hkdf;
use sha2::Sha256;
use macros::StrChoice;
use altraits::traits::StrChoice;


#[derive(Debug, Clone, StrChoice)]
#[strchoice(case="snake")]
pub enum OtpAction {
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
impl PartialEq for OtpType {
    fn eq(&self, other: &Self) -> bool {
        self.to_str() == other.to_str()
    }
}
impl OtpType {
    pub fn to_enum(value: &str) -> Self {
        match value {
            "email" => OtpType::EMAIL,
            "authenticator" => OtpType::AUTHENTICATOR,
            _ => panic!("Invalid OtpType value"),
        }
    }
}
impl Display for OtpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
impl Display for OtpAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl OtpAction {
    pub fn to_enum(value: String) -> Self {
        match value.as_str() {
            "verify_account" => OtpAction::VerifyAccount,
            "change_password" => OtpAction::ChangePassword,
            "make_transaction" => OtpAction::MakeTransaction,
            "two_fa_login" => OtpAction::TwoFaLogin,
            _ => panic!("Invalid OtpAction value"),
        }
    }
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
    pub fn derive_key(user_email: &str, action: OtpAction) -> Vec<u8> {
        let hk = Hkdf::<Sha256>::new(None, CONFIG.otp_encryption_key.as_bytes());
        let mut okm = vec![0u8; 20]; // Length for SHA1
        hk.expand(action.to_str().as_bytes(), &mut okm).expect("HKDF expand failed");
        hk.expand(user_email.as_bytes(), &mut okm).expect("User email didn't return, explicit expand failed");
        okm
    }

    /// Create a new OTP instance
    pub fn new(user_email: &str, action: OtpAction, otp_type: OtpType) -> Result<Self, String> {
        if user_email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        let derived_key_bytes = Self::derive_key(user_email, action);
        let base32_secret = encode(Alphabet::RFC4648 { padding: false }, &derived_key_bytes);
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
