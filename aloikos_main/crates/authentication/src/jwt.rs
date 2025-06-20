use chrono::{Duration, Utc};
use futures::prelude::*;
use jsonwebtoken::{
    self, decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use redis::AsyncCommands;
use std::{str::FromStr, vec};
// use redis::aio::c
use serde::{Deserialize, Serialize};
use shared::constants::{
    JWT_ACCESS_DURATION, JWT_ALGORITHM, JWT_REFRESH_DURATION, JWT_SIGNING_KEY, JWT_VERIFYING_KEY,
};

const DECODING_KEY: DecodingKey = DecodingKey::from_secret(JWT_VERIFYING_KEY.as_bytes());
const ENCODING_KEY: EncodingKey = EncodingKey::from_secret(JWT_SIGNING_KEY.as_bytes());

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtAccessToken {
    pub token: String,
}
pub struct JwtRefreshToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_type: String,
    pub jti: String,
}
#[async_trait::async_trait]
pub trait Blacklist {
    async fn blacklist(&self) -> Result<(), String>;
    async fn check_blacklist(&self, jti: String) -> impl Future<Output = Result<(), String>> {
        let client = redis::Client::open(REDIS_URL).map_err(|e| e.to_string())?;
        let mut con = client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let key = jti;
        if let Some(result) = con.get(key).await.map_err(|e| e.to_string()) {
            if result.is_some() {
                return Err("Token is blacklisted".to_string());
            }
        };
        Ok(())
    }
    fn jti(&self) -> Option<String> {
        let default_algorithm = Algorithm::from_str(JWT_ALGORITHM).unwrap();
        let mut validation = Validation::new(default_algorithm);
        validation.insecure_disable_signature_validation();
        validation.validate_exp = false;
        validation.validate_nbf = false;
        validation.validate_aud = false;
        if let token_data = decode::<Claims>(token, &DECODING_KEY, &validation) {
            return Some(token_data.claims.jti);
        }
        None
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
        let jti = uuid::Uuid::new_v4().to_string();
        let headers = Header::new(default_algorithm);
        let claims = Claims {
            sub: user_id.clone(),
            exp: (Utc::now() + Duration::seconds(JWT_ACCESS_DURATION as i64)).timestamp() as usize,
            token_type: "access".to_string(),
            jti,
        };
        let token = encode(&headers, &claims, &ENCODING_KEY).expect("Failed to encode JWT");
        JwtAccessToken { token }
    }
    fn verify(&self) -> Result<TokenData<Claims>, String> {
        let default_algorithm = Algorithm::from_str(JWT_ALGORITHM).unwrap();
        let mut validation = Validation::default();
        validation.leeway = JWT_REFRESH_DURATION as u64;
        validation.algorithms = vec![default_algorithm];
        decode::<Claims>(&self.token, &DECODING_KEY, &validation).map_err(|e| e.to_string())?;
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
        let jti = uuid::Uuid::new_v4().to_string();
        let claims = Claims {
            sub: user_id.clone(),
            exp: (Utc::now() + Duration::seconds(JWT_REFRESH_DURATION)).timestamp() as usize,
            token_type: "refresh".to_string(),
            jti,
        };
        let token = encode(&headers, &claims, &ENCODING_KEY).expect("Failed to encode JWT");
        JwtRefreshToken { token }
    }
    fn verify(&self) -> Result<TokenData<Claims>, String> {
        let default_algorithm = Algorithm::from_str(JWT_ALGORITHM).unwrap();
        let mut validation = Validation::default();
        validation.leeway = JWT_REFRESH_DURATION as u64;
        validation.algorithms = vec![default_algorithm];
        let result = decode::<Claims>(&self.token, &DECODING_KEY, &validation)
            .expect("Failed to decode JWT");
        Ok(result)
    }
}

#[async_trait::async_trait]
impl Blacklist for JwtAccessToken {
    async fn blacklist(&self) -> Result<(), String> {
        // Here you would typically add the token to a blacklist in Redis or another store
        let client = redis::Client::open(REDIS_URL).map_err(|e| e.to_string())?;
        let mut con = client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let key = self
            .jti()
            .unwrap_or_else(|| panic!("JWT does not have a valid jti (JWT ID). Cannot blacklist."));
        let _: () = con
            .set_ex(
                key,
                true,
                JWT_ACCESS_DURATION as usize,
            )
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
#[async_trait::async_trait]
impl Blacklist for JwtRefreshToken {
    async fn blacklist(&self) -> Result<(), String> {
        // Here you would typically add the token to a blacklist in Redis or another store
        let client = redis::Client::open(REDIS_URL).map_err(|e| e.to_string())?;
        let mut con = client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let key = self
            .jti()
            .unwrap_or_else(|| panic!("JWT does not have a valid jti (JWT ID). Cannot blacklist."));
        let _: () = con
            .set_ex(
                key,
                true,
                JWT_REFRESH_DURATION as usize,
            )
            .await
            .map_err(|e| e.to_string());
        Ok(())
    }
}

impl JwtAccessToken {
    pub async fn full_verify(&self) -> Result<TokenData<Claims>, String> {
        let token_data = self.verify().unwrap();
        self.check_blacklist(token_data.claims.jti.clone()).await.map_err(|e| e.to_string());
        Ok(())
    }
}

impl JwtRefreshToken {
    pub async fn full_verify(&self) -> Result<TokenData<Claims>, String> {
        let token_data = self.verify().unwrap();
        self.check_blacklist(token_data.claims.jti.clone()).await?;
        Ok(())
    }
}
