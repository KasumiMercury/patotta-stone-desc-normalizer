use std::fs::File;
use std::io::Write;
use anyhow::anyhow;
use serde::Deserialize;
use sqlx::SqlitePool;
use tempfile::TempDir;
use crate::custom_error::CustomError;

pub(crate) fn file_open(path: &str) -> Result<File, CustomError> {
    let file = File::open(path)
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to open {}: {}", path, e)))?;
    Ok(file)
}

#[test]
fn test_file_open_success() {
    // create a temporary file
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_file_open.csv");

    let mut temp_file = File::create(&path).unwrap();
    temp_file.write_all(b"source_id,title,description,published_at,actual_start_at\n").unwrap();

    // open the file test
    let file = file_open(path.to_str().unwrap()).unwrap();

    // check if the file is opened
    assert!(file.metadata().is_ok());

    // cleanup
    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_file_open_failure() {
    // open the file test
    let file = file_open("non_existent_file.csv");

    // check if the file is opened
    assert!(file.is_err());
}


#[derive(Deserialize)]
pub struct Record {
    source_id: String,
    title: String,
    description: String,
    published_at: String,
    actual_start_at: String,
}

pub(crate) fn csv_parse(file: File) -> anyhow::Result<Vec<Record>, CustomError> {
    let mut rdr = csv::Reader::from_reader(file);

    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result
            .map_err(|e| CustomError::Anyhow(anyhow!("Failed to parse CSV: {}", e)))?;
        records.push(record);
    }
    Ok(records)
}


pub(crate) async fn initialize_desc_table_by_records(
    pool: &SqlitePool,
    records: Vec<Record>,
) -> anyhow::Result<(), CustomError> {
    // if data is already present, delete it
    sqlx::query("DELETE FROM description")
        .execute(&*pool)
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to delete from desc: {}", e)))?;

    // insert new data
    for record in records {
        sqlx::query("INSERT INTO description (source_id, title, description, published_at, actual_start_at) VALUES (?, ?, ?, ?, ?)")
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
