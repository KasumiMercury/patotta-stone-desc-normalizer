#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Failed to get database path")]
    PathError,
    #[error("Failed to connect to database: {0}")]
    ConnectionError(#[from] sqlx::Error),
}
