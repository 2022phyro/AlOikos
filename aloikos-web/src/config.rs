use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

pub struct Config {
    // JWT configuration
    pub jwt_secret: String,
    pub jwt_signing_key: String,
    pub jwt_verifying_key: String,
    pub jwt_algorithm: String,
    pub jwt_access_duration: u64,
    pub jwt_refresh_duration: i64,
    pub jwt_issuer: String,
    pub redis_url: String,

    // OTP configuration
    pub otp_expiry: String,
    pub otp_digit_length: usize,
    pub otp_skew: u8,
    pub otp_issuer: String,
    pub otp_encryption_key: String,
    // Server configuration
    pub server_id: usize,
    pub server_datacenter_id: usize,
    pub server_env: String,

    //DB
    pub db_uri: String,
    pub db_name: String,
}

impl Config {
    fn new() -> Self {
        // Load environment variables from .env file
        dotenv().ok();
        
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file"),
            jwt_signing_key: env::var("JWT_SIGNING_KEY").expect("Need JWT signing key"),
            jwt_verifying_key: env::var("JWT_VERIFYING_KEY").expect("Need JWT verifying key"),
            jwt_algorithm: env::var("JWT_ALGORITHM").unwrap_or_else(|_| "HS256".to_string()),
            jwt_access_duration: env::var("JWT_ACCESS_DURATION")
                .expect("Specify JWT lifetimes")
                .parse()
                .expect("JWT_ACCESS_DURATION must be a valid u64"),
            jwt_refresh_duration: env::var("JWT_REFRESH_DURATION")
                .expect("Specify JWT lifetimes")
                .parse()
                .expect("Must be a valid number"),
            jwt_issuer: env::var("JWT_ISSUER").unwrap_or_else(|_| "Aloikos".to_string()),
            redis_url: env::var("REDIS_URL").expect("Set Redis url"),

            otp_expiry: env::var("OTP_EXPIRY").unwrap_or_else(|_| "30".to_string()),
            otp_digit_length: env::var("OTP_DIGIT_LENGTH")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .expect("OTP_DIGIT_LENGTH must be a valid usize"),
            otp_skew: env::var("OTP_SKEW")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .expect("OTP_SKEW must be a valid u64"),
            otp_issuer: env::var("OTP_ACCOUNT_NAME").unwrap_or_else(|_| "Aloikos".to_string()),
            otp_encryption_key: env::var("OTP_ENCRYPTION_KEY").unwrap_or_else(|_| "Aloikos".to_string()),
            server_id: env::var("SERVER_ID")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .expect("SERVER_ID must be a valid usize"),
            server_datacenter_id: env::var("SERVER_DATACENTER_ID")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .expect("DATACENTER_ID must be a valid usize"),
            server_env: env::var("SERVER_ENV").expect("Set server environment"),
            db_uri: env::var("DATABASE_URL").expect("Set db url"),
            db_name: env::var("DB_NAME").expect("Set db name"),

        }
    }
}

// Global singleton instance - initialized once on first access
lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

// For backward compatibility, you can also provide these functions
pub fn load_config() -> &'static Config {
    &CONFIG
}
