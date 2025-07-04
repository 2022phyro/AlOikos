use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite doesn't support modifying columns directly, so we need to:
        // 1. Create a new table with the desired schema
        // 2. Copy data from the old table
        // 3. Drop the old table
        // 4. Rename the new table

        // Step 1: Create new table with updated schema
        manager
            .create_table(
                Table::create()
                    .table(User::TempTable)
                    .if_not_exists()
                    .col(big_integer(User::Id).not_null().primary_key())
                    .col(timestamp_with_time_zone(User::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(User::UpdatedAt).not_null())
                    .col(string(User::Email).not_null().unique_key())
                    .col(string(User::FirstName).not_null())
                    .col(string(User::LastName).not_null())
                    .col(string(User::UserName).not_null().unique_key())
                    .col(timestamp_with_time_zone(User::DateOfBirth))
                    .col(timestamp_with_time_zone(User::AuthChange))
                    .col(string(User::Status).not_null().default("unverified")) // Updated with default
                    .col(string(User::Password).not_null())
                    .to_owned(),
            )
            .await?;

        // Step 2: Copy data from old table to new table
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            INSERT INTO user_temp (id, created_at, updated_at, email, first_name, last_name, user_name, date_of_birth, auth_change, status, password)
            SELECT id, created_at, updated_at, email, first_name, last_name, user_name, date_of_birth, auth_change, 
                   COALESCE(status, 'unverified') as status, password 
            FROM user
            "#,
        ).await?;

        // Step 3: Drop old table
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // Step 4: Rename new table to original name
        db.execute_unprepared("ALTER TABLE user_temp RENAME TO user").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Reverse the migration by recreating the table without the default value
        
        // Step 1: Create temp table with old schema (no default for status)
        manager
            .create_table(
                Table::create()
                    .table(User::TempTable)
                    .if_not_exists()
                    .col(big_integer(User::Id).not_null().primary_key())
                    .col(timestamp_with_time_zone(User::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(User::UpdatedAt).not_null())
                    .col(string(User::Email).not_null().unique_key())
                    .col(string(User::FirstName).not_null())
                    .col(string(User::LastName).not_null())
                    .col(string(User::UserName).not_null().unique_key())
                    .col(timestamp_with_time_zone(User::DateOfBirth))
                    .col(timestamp_with_time_zone(User::AuthChange))
                    .col(string(User::Status).not_null()) // No default value
                    .col(string(User::Password).not_null())
                    .to_owned(),
            )
            .await?;

        // Step 2: Copy data
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            INSERT INTO user_temp (id, created_at, updated_at, email, first_name, last_name, user_name, date_of_birth, auth_change, status, password)
            SELECT id, created_at, updated_at, email, first_name, last_name, user_name, date_of_birth, auth_change, status, password 
            FROM user
            "#,
        ).await?;

        // Step 3: Drop current table
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // Step 4: Rename temp table
        db.execute_unprepared("ALTER TABLE user_temp RENAME TO user").await?;

        Ok(())
    }
}

#[derive(Iden)]
enum User {
    Table,
    TempTable,
    Id,
    CreatedAt,
    UpdatedAt,
    Email,
    FirstName,
    LastName,
    UserName,
    DateOfBirth,
    AuthChange,
    Status,
    Password,
}
