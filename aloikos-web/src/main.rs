pub mod config;
use config::Config;

fn main() {
    let config = Config::new();
    
    println!("\nConfig settings \n");
    println!("JWT_ACCESS_DURATION: {}", config.jwt_access_duration);
    println!("JWT_ALGORITHM: {}", config.jwt_algorithm);
    println!("JWT_ISSUER: {}", config.jwt_issuer);
    println!("JWT_REFRESH_DURATION: {}", config.jwt_refresh_duration);
    println!("JWT_SECRET: {}", config.jwt_secret);
    println!("JWT_SIGNING_KEY: {}", config.jwt_signing_key);
    println!("JWT_VERIFYING_KEY: {}", config.jwt_verifying_key);
    println!("REDIS_URL: {}", config.redis_url);
}
