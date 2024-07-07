use std::fs::create_dir_all;
use std::path::PathBuf;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

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
pub async fn initialize_sqlite(data_path: PathBuf) -> Result<(), CustomError> {
    let db_path = db_path(data_path)?;

    // create the data dir if it does not exist
    create_dir_all(&data_path).unwrap();

    // create the sqlite database if it does not exist
    let db_exists = Sqlite::database_exists(&db_path)
        .await
        .map_err(|e| CustomError::DbError(DbError::CreateError(e)))?;
    if !db_exists {
        Sqlite::create_database(&db_path)
            .await
            .map_err(|e| CustomError::DbError(DbError::CreateError(e)))?;

        // print the path
        println!("sqlite database created at {}", db_path);
    }

    Ok(())
}
