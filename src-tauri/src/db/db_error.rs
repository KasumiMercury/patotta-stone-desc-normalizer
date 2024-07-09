#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Failed to get database path")]
    Path,
    #[error("Failed to create database: {0}")]
    Create(#[source] sqlx::Error),
    #[error("Failed to migrate database: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("Failed to connect to database: {0}")]
    Connection(#[source] sqlx::Error),
    #[error("Failed to execute query: {0}")]
    Query(#[source] sqlx::Error),
}
