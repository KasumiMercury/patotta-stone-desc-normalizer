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

pub async fn initialize_sqlite(handle: AppHandle) -> Result<(), CustomError> {
    Ok(())
}
