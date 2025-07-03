pub use sea_orm_migration::prelude::*;
mod m20250703_155520_user_group_permission;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250703_155520_user_group_permission::Migration),
        ]
    }
}
