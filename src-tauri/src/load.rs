use std::fs::File;
use anyhow::anyhow;
use crate::custom_error::CustomError;

fn file_open(path: &str) -> Result<File, CustomError> {
    let file = File::open(path)
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to open {}: {}", path, e)))?;
    Ok(file)
}
