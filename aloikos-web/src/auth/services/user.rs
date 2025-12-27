use super::super::models::prelude::{User, UserActiveModel, UserModel};
use crate::auth::dto::{UserCreateDto, UserUpdateDto};
use crate::auth::models::user::UserStatus;
use crate::authentication::password::hash_password;
use crate::db::db;
use crate::new_model;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DbErr, DeleteResult, EntityTrait, Set};
// use crate::db::load_db;
// pub async fn create(user: UserCreate) -> UserActiveModel {
//     // let user = UserActiveModel {}
// }

pub async fn create(user_data: UserCreateDto) -> Result<UserModel, DbErr> {
    let user = new_model!(
        UserActiveModel, {
            email: user_data.email,
            first_name: user_data.first_name,
            last_name: user_data.last_name,
            user_name: user_data.user_name,
            auth_change: Some(Utc::now()),
            status: UserStatus::Unverified,
            password: hash_password(&user_data.password).unwrap(),
        }
    );
    let user: UserModel = user.insert(db()).await?;
    Ok(user)
}

pub async fn get(user_id: i64) -> Result<UserModel, DbErr> {
    let user = User::find_by_id(user_id)
        .one(db())
        .await?;
    match user {
        Some(user_model) => Ok(user_model),
        None => Err(DbErr::RecordNotFound("User not found".to_owned())),
    }
}
pub async fn query_builder() {}
pub async fn filter( ) {} 
pub async fn update(user_id: i64, user_data: UserUpdateDto) -> Result<UserModel, DbErr> {
    let user = User::find_by_id(user_id).one(db()).await?;
    let mut user: UserActiveModel = match user {
        Some(u) => u.into(),
        None => return Err(DbErr::RecordNotFound("User not found".to_owned())),
    };
    if let Some(first_name) = user_data.first_name {
        user.first_name = Set(first_name);
    }
    if let Some(last_name) = user_data.last_name {
        user.last_name = Set(last_name);
    }
    if let Some(password) = user_data.password {
        user.password = Set(hash_password(&password).unwrap());
    }
    // Add more fields as needed

    let updated_user = user.update(db()).await?;
    Ok(updated_user)
}

pub async fn delete(user_id: i64) -> DeleteResult {
    let res: DeleteResult = User::delete_by_id(user_id)
        .exec(db())
        .await
        .expect("User not found");
    res
}
