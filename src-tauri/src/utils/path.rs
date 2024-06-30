use std::path::PathBuf;
use tauri::AppHandle;
use crate::utils::util_errors::UtilError;

#[allow(dead_code)]
fn app_path(handle: &AppHandle) -> Result<PathBuf, UtilError> {
    handle.path_resolver().app_data_dir()
        .ok_or(UtilError::AppDataError)
}
