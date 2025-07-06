use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::IsAdminGroup).boolean().default(false).not_null())
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .col(ColumnDef::new(Group::Description).text().not_null())
                    .col(
                        ColumnDef::new(Group::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Group::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),

            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Permission::Name).string().not_null())
                    .col(ColumnDef::new(Permission::Code).integer().not_null().unique_key())
                    .to_owned()
            )
            .await?;
        manager.create_table(
            Table::create()
            .table(User::Table).if_not_exists()
            .col(ColumnDef::new(User::Id).big_integer().not_null().primary_key())
            .col(
                ColumnDef::new(User::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(User::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(User::Email).string().not_null().unique_key())
            .col(ColumnDef::new(User::FirstName).string().not_null())
            .col(ColumnDef::new(User::LastName).string().not_null())
            .col(ColumnDef::new(User::OtpSecret).string().not_null())
            .col(ColumnDef::new(User::UserName).string().not_null().unique_key())
            .col(ColumnDef::new(User::DateOfBirth).timestamp_with_time_zone().null())
            .col(ColumnDef::new(User::AuthChange).timestamp_with_time_zone().null())
            .col(string(User::Status).not_null().default("unverified"))
            .col(ColumnDef::new(User::Password).string().not_null())
            .to_owned()
        ).await?;
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserGroup::UserId).big_integer().not_null())
                    .col(ColumnDef::new(UserGroup::GroupId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(UserGroup::UserId)
                            .col(UserGroup::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserGroup::Table, UserGroup::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserGroup::Table, UserGroup::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Group-Permission Join Table
        manager
            .create_table(
                Table::create()
                    .table(PermissionGroup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PermissionGroup::GroupId).big_integer().not_null())
                    .col(ColumnDef::new(PermissionGroup::PermissionId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(PermissionGroup::GroupId)
                            .col(PermissionGroup::PermissionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PermissionGroup::Table, PermissionGroup::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PermissionGroup::Table, PermissionGroup::PermissionId)
                            .to(Permission::Table, Permission::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for better performance
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user-email")
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user-username")
                    .table(User::Table)
                    .col(User::UserName)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user-status")
                    .table(User::Table)
                    .col(User::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes first
        manager.drop_index(Index::drop().name("idx-user-email").to_owned()).await?;
        manager.drop_index(Index::drop().name("idx-user-username").to_owned()).await?;
        manager.drop_index(Index::drop().name("idx-user-status").to_owned()).await?;
        manager.drop_table(Table::drop().table(PermissionGroup::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(UserGroup::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Permission::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Group::Table).to_owned()).await?;
        Ok(())
    }
}
#[derive(DeriveIden)]
enum Permission {
    Table,
    Id,
    Code,
    Name,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Group {
    Table,
    Id,
    Name,
    Description,
    IsAdminGroup,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    FirstName,
    LastName,
    UserName,
    DateOfBirth,
    Status,
    AuthChange,
    Password,
    CreatedAt,
    UpdatedAt,
    OtpSecret
}

#[derive(Iden)]
enum PermissionGroup {
    Table,
    GroupId,
    PermissionId,
}

#[derive(Iden)]
enum UserGroup {
    Table,
    UserId,
    GroupId,
}
