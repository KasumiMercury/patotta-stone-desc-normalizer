// Define Custom Error
use crate::db::db_error::DbError;

#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("Database error: {0}")]
    DbError(#[from] DbError),

    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
}
