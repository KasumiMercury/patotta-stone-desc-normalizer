use std::path::PathBuf;
use tauri::AppHandle;
use crate::utils::util_errors::UtilError;

const DB_NAME: &str = "data.db";

fn app_path(handle: &AppHandle) -> Result<PathBuf, UtilError> {
    handle.path_resolver().app_data_dir()
        .ok_or(UtilError::AppDataError)
}

fn db_path(mut base: PathBuf) -> String {
    base.push(DB_NAME);

    format!("sqlite://{}", base.to_str().expect("Failed to get db path"))
}
