use std::path::PathBuf;
use tauri::AppHandle;

const DB_NAME: &str = "data.db";

fn app_path(handle: &AppHandle) -> PathBuf {
    handle
        .app_data_dir()
        .expect("Failed to get app path");

    println!("app path: {:?}", app_path);

    app_path
}

fn db_path(mut base: PathBuf) -> String {
    base.push(DB_NAME);

    format!("sqlite://{}", base.to_str().expect("Failed to get db path"))
}
