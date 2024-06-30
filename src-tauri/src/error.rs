use thiserror::Error;
use crate::db::db_error::DbError;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Failed to connect to database: {0}")]
    DbError(#[from] DbError),
}
