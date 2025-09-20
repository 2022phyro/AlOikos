use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserCreateDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 100))]
    pub user_name: String,
    #[validate(length(min = 1, max = 255))]
    pub first_name: String,
    #[validate(length(min = 1, max = 255))]
    pub last_name: String,
    #[validate(length(min = 4, max = 255))]
    pub password: String,
}

pub struct UserUpdateDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
}
pub struct GroupUpdateDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_admin_group: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]

pub struct  UserFilterDto {
    email: String, 
}
pub struct LoginResultDto {
    pub access: String,
    pub refresh: String,
    pub access_expiry: DateTime<Utc>,
    pub refresh_expiry: DateTime<Utc>,
    pub user_id: String,
}
#[derive(Deserialize, ToSchema)]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}
#[derive(serde::Serialize, ToSchema)]
pub struct LoginResponse {
    pub access: String,
    pub access_expiry: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LogoutDto {
    pub all: bool
}

#[derive(Deserialize, ToSchema)]
pub struct OtpRequestDto {
    pub otp_type: Option<String>,
    pub email: String,
    pub device_id: String, // UUid for identifyng devices
}
#[derive(Deserialize, ToSchema)]
pub struct OtpVerifyDto {
    pub otp_type: Option<String>,
    pub email: String,
    pub device_id: String, // UUid for identifyng devices
    pub otp: String,
}


#[derive(Deserialize, ToSchema)]
pub struct ChangePasswordDto {
    pub email: String,
    pub password: String,
}

pub struct VerifyAccountDto {
    pub email: String,
}