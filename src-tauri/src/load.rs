use std::fs::File;
use std::io::Write;
use anyhow::anyhow;
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

