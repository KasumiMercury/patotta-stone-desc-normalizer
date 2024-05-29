// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate csv;

use std::fs::File;
use anyhow::{anyhow, Context, Result};

// Define Custom Error
#[derive(Debug, thiserror::Error)]
enum CsvError {
    #[error("IO Error: {0}")]
    Io(String),
    #[error("CSV Error: {0}")]
    Csv(String),
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn file_open(path: &str) -> Result<File>{
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(anyhow!(CsvError::Io(e.to_string()))
            .context(format!("Failed to open file: {}", path))
        )
    }
}

fn csv_parse(file: File) -> Result<()> {
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        match result {
            Ok(record) => {
                println!("{:?}", record);
            }
            Err(e) => {
                return Err(anyhow!(CsvError::Csv(e.to_string()))
                    .context("Failed to parse CSV record"));
            }
        }
    }
    Ok(())
}

fn load_csv(path: &str) -> Result<()> {
    let file = file_open(path)?;
    csv_parse(file)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
