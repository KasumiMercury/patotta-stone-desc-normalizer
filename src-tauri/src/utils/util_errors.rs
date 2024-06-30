#[derive(Debug, thiserror::Error)]
pub enum UtilError {
    #[error("Failed to get app data directory")]
    AppDataError,
}
