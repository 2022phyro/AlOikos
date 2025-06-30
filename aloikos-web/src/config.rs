use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

pub struct Config {
    pub jwt_secret: String,
    pub jwt_signing_key: String,
    pub jwt_verifying_key: String,
    pub jwt_algorithm: String,
    pub jwt_access_duration: u64,
    pub jwt_refresh_duration: i64,
    pub jwt_issuer: String,
    pub redis_url: String,
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
