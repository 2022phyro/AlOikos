use crate::{
    auth::dto::LoginResultDto,
    authentication::{
        jwt::{Claims, JwtAccessToken, JwtRefreshToken, Token},
        password::verify_password,
    },
    config::CONFIG,
    db::db,
};
use chrono::{Duration, Utc};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

use super::super::models::prelude::{User, UserColumn};

pub enum TokenType {
    Access(String),
    Refresh(String),
}
pub async fn login(email: &str, password: String) -> Result<LoginResultDto, DbErr> {
    let user = User::find()
        .filter(UserColumn::Email.eq(email))
        .one(db())
        .await?;
    
    let user = user.ok_or_else(|| {
        DbErr::RecordNotFound("User with email not found. Please recheck your email".to_owned())
    })?;
    let is_valid: bool = verify_password(&password, &user.password)
        .map_err(|_| DbErr::Custom("Invalid password. Try again later".to_owned()))?;
    if !is_valid {
        return Err(DbErr::Custom("Invalid password".to_owned()));
    }
    let access = JwtAccessToken::new(user.id.to_string(), user.auth_change.unwrap());
    let refresh: JwtRefreshToken = JwtRefreshToken::new(user.id.to_string(), user.auth_change.unwrap());
    let access_expiry = Utc::now() + Duration::seconds(CONFIG.jwt_access_duration as i64);
    let refresh_expiry = Utc::now() + Duration::seconds(CONFIG.jwt_refresh_duration);
    Ok(LoginResultDto {
        access: access.token,
        refresh: refresh.token,
        access_expiry,
        refresh_expiry,
        user_id: user.id.to_string(),
    })
}

pub async fn refresh(token: String) -> Result<LoginResultDto, DbErr> {
    let typed_token = TokenType::Refresh(token);
    let claims = verify_token(typed_token).await;
    match claims {
        Ok(claims) => {
            let access = JwtAccessToken::new(claims.sub.clone(), claims.auth_change.clone());
            let refresh: JwtRefreshToken =
                JwtRefreshToken::new(claims.sub.clone(), claims.auth_change.clone());
            let access_expiry = Utc::now() + Duration::seconds(CONFIG.jwt_access_duration as i64);
            let refresh_expiry = Utc::now() + Duration::seconds(CONFIG.jwt_refresh_duration);
            return Ok(LoginResultDto {
                access: access.token,
                refresh: refresh.token,
                access_expiry,
                refresh_expiry,
                user_id: claims.sub,
            });
        }
        Err(reason) => return Err(reason),
    }
}

pub async fn verify_token(token: TokenType) -> Result<Claims, DbErr> {
    let claims_result = match token {
        TokenType::Access(raw_token) => {
            let set_token = JwtAccessToken { token: raw_token };
            set_token.full_verify().await
        }
        TokenType::Refresh(raw_token) => {
            let set_token = JwtRefreshToken { token: raw_token };
            set_token.full_verify().await
        }
    };
    match claims_result {
        Ok(claims) => {
            if claims.claims.auth_change > Utc::now() {
                return Err(DbErr::Custom("There was a recent auth_change".to_owned()));
            } else {
                return Ok(claims.claims);
            }
        }
        Err(reason) => return Err(DbErr::Custom(reason)),
    }
}
