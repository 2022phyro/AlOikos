use super::super::models::prelude::{User, UserActiveModel, UserColumn};
use crate::auth::dto::UserCreate;
use crate::db::DB;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, DbErr};
use sea_orm::prelude::*;
use crate::auth::models::user;
use crate::authentication::password::{hash_password, verify_password};
use crate::new_model;
// use crate::db::load_db;
// pub async fn create(user: UserCreate) -> UserActiveModel {
//     // let user = UserActiveModel {}
// }

pub async fn create(user_data: UserCreate) -> User {
    let user = new_model!(
        UserActiveModel, {
            email: user_data.email,
            first_name: user_data.first_name,
            last_name: user_data.last_name,
            user_name: user_data.user_name,
            auth_change: Some(Utc::now()),
            status: user::UserStatus::Unverified,
            password: hash_password(&user_data.password).unwrap()
        }
    );
    let user: User = user.insert(&DB).await?;
    user
}