use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserCreateDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min=1, max=100))]
    pub user_name: String,
    #[validate(length(min=1, max=255))]
    pub first_name: String,
    #[validate(length(min=1, max=255))]
    pub last_name: String,
    #[validate(length(min=4, max=255))]
    pub password: String,
}

pub struct UserUpdateDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
}