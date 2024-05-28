// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate csv;

use std::error::Error;
use std::fs::File;
use anyhow::{anyhow, Context, Result as AnyHowResult};
use thiserror::Error;

// Define Custom Error
#[derive(Debug, Error)]
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

fn file_open(path: &str) -> Result<File, Box<dyn Error>>{
    let file = File::open(path)?;
    Ok(file)
}

fn csv_parse(file: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn load_csv(path: &str) -> Result<(), Box<dyn Error>> {
    match csv_parse(file_open(&path).unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
