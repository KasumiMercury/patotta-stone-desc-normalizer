use thiserror::Error;
use crate::db::db_error::DbError;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error(transparent)]
    DbError(#[from] DbError),
}
