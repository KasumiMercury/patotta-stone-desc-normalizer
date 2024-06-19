// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{anyhow, Context as _, Result};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use tauri::{Manager, State};

use custom_error::CustomError;

mod custom_error;
mod load;

impl serde::Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn get_sqlite_pool() -> Result<SqlitePool, CustomError> {
    let path = std::path::Path::new("sqlite.db");
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options)
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to connect to database: {}", e)))?;

    Ok(pool)
}

#[tauri::command]
async fn load_csv(pool: State<'_, SqlitePool>, path: &str) -> Result<(), CustomError> {
    let file = load::file_open(path).context("Failed to open file")?;
    let records = load::csv_parse(file).context("Failed to parse CSV")?;

    // initialize the desc table with the records
    load::initialize_desc_table_by_records(&pool, records).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
struct Description {
    pub id: i32,
    pub source_id: String,
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub actual_start_at: String,
}

#[tauri::command]
async fn get_description_by_source_id(
    pool: State<'_, SqlitePool>,
    source_id: String,
) -> Result<String, CustomError> {
    get_description_by_source_id_infra(pool, &source_id)
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to get description by source_id: {}", e)))
        .map(|desc| serde_json::to_string(&desc).unwrap())
}

async fn get_description_by_source_id_infra(
    pool: State<'_, SqlitePool>,
    source_id: &str,
) -> Result<Description, sqlx::Error> {
    // get pool from the state
    let p = pool.clone();
    // get description by source_id from the sqlite database
    let desc = sqlx::query_as::<_, Description>(
        r#"
        SELECT * FROM desc WHERE source_id = ?
        "#,
    )
    .bind(source_id)
    .fetch_one(&*p)
    .await?;

    Ok(desc)
}

fn main() {
    use tauri::async_runtime::block_on;

    dotenv().expect("Failed to load .env file");
    let pool = block_on(get_sqlite_pool()).expect("Failed to create SQLite pool");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_csv,
            get_description_by_source_id
        ])
        .setup(|app| {
            app.manage(pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
