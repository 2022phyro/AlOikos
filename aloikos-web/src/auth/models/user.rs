use sea_orm::entity::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum UserStatus {
    #[sea_orm(string_value = "unverified")]
    Unverified,
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "banned")]
    Banned,
    #[sea_orm(string_value = "deactivated")]
    Deactivated,
    #[sea_orm(string_value = "in review")]
    Review,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    #[sea_orm(unique)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(unique)]
    pub user_name: String,
    pub date_of_birth: Option<DateTimeUtc>,
    pub auth_change: Option<DateTimeUtc>,
    pub status: UserStatus,
    pub password: String,
    pub otp_secret: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_group::Entity")]
    UserGroup,
}

impl Related<super::user_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserGroup.def()
    }
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_group::Relation::Group.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_group::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {
    
}
