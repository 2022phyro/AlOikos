use chrono::{Duration, Utc};
use jsonwebtoken::{
    self, decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use redis::AsyncCommands;
use sea_orm::prelude::DateTimeUtc;
use std::{str::FromStr, vec};
use serde::{Deserialize, Serialize};
use crate::{config::CONFIG};

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
    pub auth_change: DateTimeUtc,
}

pub trait Token {
    fn from_token(token: String) -> Self;
    fn new(user_id: String, auth_change: DateTimeUtc) -> Self;
    fn verify(&self) -> Result<TokenData<Claims>, String>;
    fn signing_key() -> EncodingKey {
        EncodingKey::from_secret(CONFIG.jwt_signing_key.as_bytes())
    }
    fn verifying_key() -> DecodingKey {
        DecodingKey::from_secret(CONFIG.jwt_verifying_key.as_bytes())
    }
}

#[async_trait::async_trait]
pub trait Blacklist {
    async fn blacklist(&self) -> Result<(), String>;
    async fn check_blacklist(&self, jti: String) -> Result<(), String> {
        let client = redis::Client::open(CONFIG.redis_url.as_str()).map_err(|e| e.to_string())?;
        let mut con = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let key = jti;
        let result: Option<bool> = con.get(&key).await.map_err(|e| e.to_string())?;
        if result.is_some() {
            return Err("Token is blacklisted".to_string());
        }
        Ok(())
    }
    fn jti(&self) -> Option<String>;
}


impl Token for JwtAccessToken {
    fn from_token(token: String) -> Self {
        JwtAccessToken { token }
    }
    fn new(user_id: String, auth_change: DateTimeUtc) -> Self {
        // Here you would typically generate a JWT token based on user_id and token_type
        let default_algorithm = Algorithm::from_str(&CONFIG.jwt_algorithm).unwrap();
        let jti = uuid::Uuid::new_v4().to_string();
        let headers = Header::new(default_algorithm);
        let claims = Claims {
            sub: user_id.clone(),
            exp: (Utc::now() + Duration::seconds(CONFIG.jwt_access_duration as i64)).timestamp() as usize,
            token_type: "access".to_string(),
            auth_change:auth_change,
            jti,
        };
        let token = encode(&headers, &claims, &Self::signing_key()).expect("Failed to encode JWT");
        JwtAccessToken { token }
    }
    fn verify(&self) -> Result<TokenData<Claims>, String> {
        let default_algorithm = Algorithm::from_str(&CONFIG.jwt_algorithm).unwrap();
        let mut validation = Validation::default();
        validation.leeway = CONFIG.jwt_refresh_duration as u64;
        validation.algorithms = vec![default_algorithm];
        let result = decode::<Claims>(&self.token, &Self::verifying_key(), &validation).map_err(|e| e.to_string())?;
        Ok(result)
    }
}

impl Token for JwtRefreshToken {
    fn from_token(token: String) -> Self {
        JwtRefreshToken { token }
    }
    fn new(user_id: String, auth_change: DateTimeUtc) -> Self {
        // Here you would typically generate a JWT token based on user_id and token_type
        let default_algorithm = Algorithm::from_str(&CONFIG.jwt_algorithm).unwrap();
        let headers = Header::new(default_algorithm);
        let jti = uuid::Uuid::new_v4().to_string();
        let claims = Claims {
            sub: user_id.clone(),
            exp: (Utc::now() + Duration::seconds(CONFIG.jwt_refresh_duration)).timestamp() as usize,
            token_type: "refresh".to_string(),
            auth_change,
            jti,
        };
        let token = encode(&headers, &claims, &Self::signing_key()).expect("Failed to encode JWT");
        JwtRefreshToken { token }
    }
    fn verify(&self) -> Result<TokenData<Claims>, String> {
        let default_algorithm = Algorithm::from_str(&CONFIG.jwt_algorithm).unwrap();
        let mut validation = Validation::default();
        validation.leeway = CONFIG.jwt_refresh_duration as u64;
        validation.algorithms = vec![default_algorithm];
        let result = decode::<Claims>(&self.token, &Self::verifying_key(), &validation)
            .map_err(|e| e.to_string())?;
        Ok(result)
    }
}

#[async_trait::async_trait]
impl Blacklist for JwtAccessToken {
    async fn blacklist(&self) -> Result<(), String> {
        // Here you would typically add the token to a blacklist in Redis or another store
        let client = redis::Client::open(CONFIG.redis_url.as_str()).map_err(|e| e.to_string())?;
        let mut con = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let key = self
            .jti()
            .unwrap_or_else(|| panic!("JWT does not have a valid jti (JWT ID). Cannot blacklist."));
        let _: () = con
            .set_ex(
                key,
                true,
                CONFIG.jwt_access_duration as u64,
            )
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn jti(&self) -> Option<String> {
        let default_algorithm = Algorithm::from_str(&CONFIG.jwt_algorithm).unwrap();
        let mut validation = Validation::new(default_algorithm);
        validation.insecure_disable_signature_validation();
        validation.validate_exp = false;
        validation.validate_nbf = false;
        validation.validate_aud = false;
        if let Ok(token_data) = decode::<Claims>(&self.token, &Self::verifying_key(), &validation) {
            return Some(token_data.claims.jti);
        }
        None
    }
}
#[async_trait::async_trait]
impl Blacklist for JwtRefreshToken {
    async fn blacklist(&self) -> Result<(), String> {
        // Here you would typically add the token to a blacklist in Redis or another store
        let client = redis::Client::open(CONFIG.redis_url.as_str()).map_err(|e| e.to_string())?;
        let mut con = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let key = self
            .jti()
            .unwrap_or_else(|| panic!("JWT does not have a valid jti (JWT ID). Cannot blacklist."));
        let _: () = con
            .set_ex(
                key,
                true,
                CONFIG.jwt_refresh_duration as u64,
            )
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn jti(&self) -> Option<String> {
        let default_algorithm = Algorithm::from_str(&CONFIG.jwt_algorithm).unwrap();
        let mut validation = Validation::new(default_algorithm);
        validation.insecure_disable_signature_validation();
        validation.validate_exp = false;
        validation.validate_nbf = false;
        validation.validate_aud = false;
        if let Ok(token_data) = decode::<Claims>(&self.token, &Self::verifying_key(), &validation) {
            return Some(token_data.claims.jti);
        }
        None
    }
}

impl JwtAccessToken {
    pub async fn full_verify(&self) -> Result<TokenData<Claims>, String> {
        let token_data = self.verify()?;
        self.check_blacklist(token_data.claims.jti.clone()).await?;
        Ok(token_data)
    }
}

impl JwtRefreshToken {
    pub async fn full_verify(&self) -> Result<TokenData<Claims>, String> {
        let token_data = self.verify()?;
        self.check_blacklist(token_data.claims.jti.clone()).await?;
        Ok(token_data)
    }
}
