#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Failed to connect to database: {0}")]
    ConnectionError(#[from] sqlx::Error),
}
