// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;

use anyhow::{anyhow, Context as _, Result};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;

use custom_error::CustomError;

use serde::Deserialize;

mod custom_error;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
    Ok((records))
}

async fn initialize_desc_table_by_records(pool: &SqlitePool, records: Vec<Record>) -> Result<(), CustomError> {
    let mut conn = pool.acquire().await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to acquire connection: {}", e)))?;

    // if data is already present, delete it
    sqlx::query("DELETE FROM desc")
        .execute(&mut conn)
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to delete records: {}", e)))?;

    // initialize the table with the new data
    for record in records {
        sqlx::query(
            r#"
            INSERT INTO desc (source_id, title, description, published_at, actual_start_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(record.source_id)
        .bind(record.title)
        .bind(record.description)
        .bind(record.published_at)
        .bind(record.actual_start_at)
        .execute(&mut conn)
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to insert record: {}", e)))?;
    }
    Ok(())
}

fn load_csv(path: &str) -> Result<(), CustomError> {
    let file = file_open(path)
        .context("Failed to open file")?;
    let records = csv_parse(file)
    .context("Failed to parse CSV")?;

    for record in records {
        println!("{} : {}", record.source_id, record.title);
    }
    Ok(())
}

async fn create_sqlite_pool() -> Result<sqlx::SqlitePool, CustomError> {
    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL must be set")?;
    let pool = SqlitePool::connect(&database_url).await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to create SQLite pool: {}", e)))?;
    Ok(pool)
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");
    create_sqlite_pool().await.expect("Failed to create SQLite pool");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("world"), "Hello, world! You've been greeted from Rust!");
    }

    #[test]
    fn test_load_csv() {
        let result = load_csv("test.csv");
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_invalid_csv() {
        let result = load_csv("invalid.csv");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = load_csv("nonexistent.csv");
        assert!(result.is_err());
    }
}
