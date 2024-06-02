// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{anyhow, Context as _, Result};
use custom_error::CustomError;
use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::fs::File;
use tauri::{Manager, State};

mod custom_error;

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
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| CustomError::Anyhow(anyhow!("DATABASE_URL must be set")))?;
    let pool = SqlitePool::connect(&database_url)
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to create SQLite pool: {}", e)))?;

    Ok(pool)
}

fn file_open(path: &str) -> Result<File, CustomError> {
    let file = File::open(path)
        .map_err(|e| CustomError::Anyhow(anyhow!("File Error: {}", e)))
        .with_context(|| format!("Failed to open file: {}", path))?;
    Ok(file)
}

#[derive(Deserialize)]
struct Record {
    source_id: String,
    title: String,
    description: String,
    published_at: String,
    actual_start_at: String,
}

fn csv_parse(file: File) -> Result<Vec<Record>, CustomError> {
    let mut rdr = csv::Reader::from_reader(file);

    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result
            .map_err(|e| CustomError::Anyhow(anyhow!("CSV Error: {}", e)))
            .with_context(|| "Failed to parse CSV record")?;
        records.push(record);
    }
    Ok(records)
}

async fn initialize_desc_table_by_records(
    pool: &SqlitePool,
    records: Vec<Record>,
) -> Result<(), CustomError> {
    // if data is already present, delete it
    sqlx::query("DELETE FROM desc")
        .execute(&*pool)
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to delete from desc: {}", e)))?;

    // insert new data
    for record in records {
        sqlx::query("INSERT INTO desc (source_id, title, description, published_at, actual_start_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&record.source_id)
            .bind(&record.title)
            .bind(&record.description)
            .bind(&record.published_at)
            .bind(&record.actual_start_at)
            .execute(&*pool)
            .await
            .map_err(|e| CustomError::Anyhow(anyhow!("Failed to insert into desc: {}", e)))?;
    }
    Ok(())
}

#[tauri::command]
async fn load_csv(pool:State<'_, SqlitePool>, path: &str) -> Result<(), CustomError> {
    let file = file_open(path).context("Failed to open file")?;
    let records = csv_parse(file).context("Failed to parse CSV")?;

    // initialize the desc table with the records
    initialize_desc_table_by_records(&*pool, records).await?;
    Ok(())
}

fn main() {
    use tauri::async_runtime::block_on;

    dotenv().expect("Failed to load .env file");
    let pool = block_on(get_sqlite_pool()).expect("Failed to create SQLite pool");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_csv])
        .setup(|app| {
            app.manage(pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
