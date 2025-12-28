use sea_orm::entity::prelude::*;
use crate::users::models;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub is_admin_group: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::permission_group::Entity")]
    PermissionGroup,
    #[sea_orm(has_many = "models::user_group::Entity")]
    UserGroup,
}

impl Related<super::permission_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermissionGroup.def()
    }
}

impl Related<models::user_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserGroup.def()
    }
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        super::permission_group::Relation::Permission.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::permission_group::Relation::Group.def().rev())
    }
}

impl Related<models::user::Entity> for Entity {
    fn to() -> RelationDef {
        models::user_group::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(models::user_group::Relation::Group.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
