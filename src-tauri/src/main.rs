// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod custom_error;

extern crate csv;

use std::fs::File;
use anyhow::{anyhow, Context as _, Result};
use custom_error::CustomError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn file_open(path: &str) -> Result<File, CustomError>{
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(CustomError::IoError(e))
    }
}

fn csv_parse(file: File) -> Result<(), CustomError> {
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        match result {
            Ok(record) => {
                println!("{:?}", record);
            }
            Err(e) => {
                return Err(
                    CustomError::CsvReaderError(e)
                        .context("Failed to read CSV file")
                );
            }
        }
    }
    Ok(())
}

fn load_csv(path: &str) -> Result<(), CustomError> {
    let file = file_open(path)
        .context("Failed to open file")?;
    csv_parse(file)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
