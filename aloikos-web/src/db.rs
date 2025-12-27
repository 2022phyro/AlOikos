use once_cell::sync::OnceCell;
use crate::config::CONFIG;
use sea_orm::{ Database, DbErr, DatabaseConnection};

pub static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn connect_db() -> Result<(), DbErr> {
    let db = Database::connect(&CONFIG.db_uri).await?;
    DB.set(db).map_err(|_| DbErr::Custom("DB already initialized".into()))?;
    Ok(())
}

pub fn db() -> &'static DatabaseConnection {
    DB.get().expect("Database not initialized. Call connect_db() first.")
}
