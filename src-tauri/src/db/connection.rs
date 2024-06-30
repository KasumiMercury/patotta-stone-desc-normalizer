use std::path::PathBuf;
use sqlx::SqlitePool;
use tauri::AppHandle;

use crate::error::CustomError;
use crate::db::db_error::DbError;

const DB_NAME: &str = "data.db";

fn db_path(mut base: PathBuf) -> String {
    base.push(DB_NAME);

    format!("sqlite://{}", base.to_str().expect("Failed to get db path"))
}

async fn get_sqlite_pool(db_path: String) -> Result<SqlitePool, CustomError> {
    let pool = SqlitePool::connect(&db_path)
        .await
        .map_err(|e| CustomError::DbError(DbError::ConnectionError(e)))?;

    Ok(pool)
}

pub async fn initialize_sqlite(handle: AppHandle) -> Result<(), CustomError> {
    Ok(())
}
