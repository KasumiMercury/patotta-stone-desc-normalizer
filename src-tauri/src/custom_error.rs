// Define Custom Error
#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("IO Error: {0}")]
    IoError(std::io::Error),
    #[error("CSV Error: {0}")]
    CsvReaderError(csv::Error),
}
