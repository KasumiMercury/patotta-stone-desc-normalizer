use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::error::CustomError;
use crate::db::db_error::DbError;

pub async fn initialize_sqlite(handle: AppHandle) -> Result<(), CustomError> {
    Ok(())
}
