use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserCreate {
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