use std::{ str::FromStr, vec};
use chrono::{Duration, Utc};
// use futures::prelude::*;
use jsonwebtoken::{
    self, decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
// use redis::*;
use serde::{Deserialize, Serialize};
use shared::constants::{
    JWT_ACCESS_DURATION, JWT_ALGORITHM, JWT_REFRESH_DURATION, JWT_SIGNING_KEY, JWT_VERIFYING_KEY,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtAccessToken {
    pub token: String,
}
pub struct JwtRefreshToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,        // Subject (user ID)
    exp: usize,         // Expiration time as a timestamp
    token_type: String, // Type of token (e.g., "access", "refresh")
}
pub trait Blacklist {
    fn blacklist(&self) -> Result<(), String>;
    fn check_blacklist(&self) -> Result<(), String> {
        Ok(())
    }
}

pub trait Token {
    fn from_token(token: String) -> Self;
    fn new(user_id: String) -> Self;
    fn verify(&self) -> Result<TokenData<Claims>, String>;
}

impl Token for JwtAccessToken {
    fn from_token(token: String) -> Self {
        JwtAccessToken { token }
    }
    fn new(user_id: String) -> Self {
        // Here you would typically generate a JWT token based on user_id and token_type
                let default_algorithm = Algorithm::from_str(JWT_ALGORITHM).unwrap();

        let headers = Header::new(default_algorithm);
        let claims = Claims {
            sub: user_id.clone(),
            exp: (Utc::now() + Duration::seconds(JWT_ACCESS_DURATION as i64)).timestamp()
                as usize,
            token_type: "access".to_string(),
        };
        let key = EncodingKey::from_secret(JWT_SIGNING_KEY.as_bytes());
        let token =
            encode(&headers, &claims, &key).expect("Failed to encode JWT");
        JwtAccessToken { token }
    }
    fn verify(&self) -> Result<TokenData<Claims>, String> {
        let default_algorithm = Algorithm::from_str(JWT_ALGORITHM).unwrap();
        let mut validation = Validation::default();
        validation.leeway = JWT_REFRESH_DURATION as u64;
        validation.algorithms = vec![default_algorithm];
        let decoding_key = DecodingKey::from_secret(JWT_VERIFYING_KEY.as_bytes());
        let result = decode::<Claims>(&self.token, &decoding_key, &validation)
            .expect("Failed to decode JWT");
        Ok(result)
    }
}

impl Token for JwtRefreshToken {
    fn from_token(token: String) -> Self {
        JwtRefreshToken { token }
    }
    fn new(user_id: String) -> Self {
        // Here you would typically generate a JWT token based on user_id and token_type
        let default_algorithm = Algorithm::from_str(JWT_ALGORITHM).unwrap();
        let headers = Header::new(default_algorithm);
        let claims = Claims {
            sub: user_id.clone(),
            exp: (Utc::now() + Duration::seconds(JWT_REFRESH_DURATION)).timestamp() as usize,
            token_type: "refresh".to_string(),
        };
        let key = EncodingKey::from_secret(JWT_SIGNING_KEY.as_bytes());
        let token = encode(&headers, &claims, &key).expect("Failed to encode JWT");
        JwtRefreshToken { token }
    }
    fn verify(&self) -> Result<TokenData<Claims>, String> {
        let default_algorithm = Algorithm::from_str(JWT_ALGORITHM).unwrap();
        let mut validation = Validation::default();
        validation.leeway = JWT_REFRESH_DURATION as u64;
        validation.algorithms = vec![default_algorithm]; 
        let decoding_key = DecodingKey::from_secret(JWT_VERIFYING_KEY.as_bytes());
        let result = decode::<Claims>(&self.token, &decoding_key, &validation)
            .expect("Failed to decode JWT");
        Ok(result)
    }
}
