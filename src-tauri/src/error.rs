use crate::db::db_error::DbError;
use crate::utils::util_errors::UtilError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error(transparent)]
    UtilError(#[from] UtilError),
    #[error(transparent)]
    DbError(#[from] DbError),
}
