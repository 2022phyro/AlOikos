use crate::config::CONFIG;
use sea_orm::{ Database, DbErr};
pub async fn connect_db()-> Result<(), DbErr> {
    let _db = Database::connect(&CONFIG.db_uri).await?;
    Ok(())
}