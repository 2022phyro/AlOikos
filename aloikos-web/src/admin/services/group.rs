use super::super::{
    dto::GroupUpdateDto,
    models::prelude::{Group, GroupActiveModel, GroupModel},
};
use crate::{db::db, new_model};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, DeleteResult, EntityTrait};

pub async fn create(
    name: String,
    description: String,
    is_admin_group: bool,
) -> Result<GroupModel, DbErr> {
    let group = new_model!(
        GroupActiveModel, {
            name: name,
            description: description,
            is_admin_group: is_admin_group,
        }
    );
    let group = group.insert(db()).await?;
    Ok(group)
}
pub async fn get(group_id: i64) -> Result<GroupModel, DbErr> {
    let group = Group::find_by_id(group_id).one(db()).await?;
    match group {
        Some(group_model) => Ok(group_model),
        None => Err(DbErr::RecordNotFound("Group not found".to_owned())),
    }
}
pub async fn update(group_id: i64, group_data: GroupUpdateDto) -> Result<GroupModel, DbErr> {
    let group = Group::find_by_id(group_id).one(db()).await?;
    let mut group: GroupActiveModel = match group {
        Some(g) => g.into(),
        None => return Err(DbErr::RecordNotFound("Group not found".to_owned())),
    };
    if let Some(name) = group_data.name {
        group.name = Set(name);
    }
    if let Some(description) = group_data.description {
        group.description = Set(description);
    }
    if let Some(is_admin_group) = group_data.is_admin_group {
        group.is_admin_group = Set(is_admin_group);
    }
    let updated_group = group.update(db()).await?;
    Ok(updated_group)
}
pub async fn delete(group_id: i64) -> DeleteResult {
    let res: DeleteResult = Group::delete_by_id(group_id)
        .exec(db())
        .await
        .expect("User not found");
    res
}
pub async fn query() {}
