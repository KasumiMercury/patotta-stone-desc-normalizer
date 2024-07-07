#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Failed to get database path")]
    PathError,
    #[error("Failed to create database: {0}")]
    CreateError(#[source] sqlx::Error),
    #[error("Failed to migrate database: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error("Failed to connect to database: {0}")]
    ConnectionError(#[source] sqlx::Error),
}
