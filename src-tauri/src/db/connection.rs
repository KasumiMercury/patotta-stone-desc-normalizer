use std::path::PathBuf;
use sqlx::SqlitePool;

use crate::error::CustomError;
use crate::db::db_error::DbError;

const DB_NAME: &str = "data.db";

#[allow(dead_code)]
fn db_path(mut base: PathBuf) -> Result<String, CustomError> {
    base.push(DB_NAME);

    // add sqlite:// to the path
    let db_path = format!("sqlite://{}", base.to_str().ok_or(CustomError::DbError(DbError::PathError))?);

    Ok(db_path)
}

#[allow(dead_code)]
async fn get_sqlite_pool(db_path: String) -> Result<SqlitePool, CustomError> {
    let pool = SqlitePool::connect(&db_path)
        .await
        .map_err(|e| CustomError::DbError(DbError::ConnectionError(e)))?;

    Ok(pool)
}

#[allow(dead_code)]
pub async fn initialize_sqlite() -> Result<(), CustomError> {
    Ok(())
}
