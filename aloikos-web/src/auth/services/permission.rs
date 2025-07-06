use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, DeleteResult, EntityTrait};
use crate::db::db;
use super::super::models::prelude::{Permission, PermissionActiveModel, PermissionModel};
use crate::utils::snowflakes::new_id;
// create
pub async fn create(name:String, code:i32) -> Result<PermissionModel, DbErr> {
    let permission = PermissionActiveModel {
            id: Set(new_id()),
            created_at: Set(chrono::Utc::now()),
            name: Set(name),
            code: Set(code) 
        };
    let permission = permission.insert(db()).await?;
    Ok(permission)
}

pub async fn get(permission_id: i64) ->  Result<PermissionModel, DbErr> {
    let permission = Permission::find_by_id(group_id)
    .one(db())
    .await?;
    match permission {
        Some(permission_model) => Ok(permission_model),
        None => Err(DbErr::RecordNotFound("Permission not found".to_owned()))
    }
}  
}
pub async fn delete(permission_id: i64) -> DeleteResult {
    let res: DeleteResult = Permission::delete_by_id(permission_id)
        .exec(db())
        .await
        .expect("Permission not found");
    res
}