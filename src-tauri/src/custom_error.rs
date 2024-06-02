// Define Custom Error
#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
