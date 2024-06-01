// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;

use anyhow::{anyhow, Context as _, Result};
use dotenvy::dotenv;

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

fn csv_parse(file: File) -> Result<(), CustomError> {
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: Record = result
            .map_err(|e| CustomError::Anyhow(anyhow!("CSV Error: {}", e)))
            .with_context(|| "Failed to parse CSV record")?;
        println!("{:?}", record.source_id);
    }
    Ok(())
}

fn load_csv(path: &str) -> Result<(), CustomError> {
    let file = file_open(path)
        .context("Failed to open file")?;
    csv_parse(file)
}

fn main() {
    dotenv().expect("Failed to load .env file");
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
