// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;
use std::path::PathBuf;

use anyhow::{anyhow, Context as _, Result};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Pool, Sqlite};
use tauri::{AppHandle, Manager, RunEvent, State};

use custom_error::CustomError;

mod custom_error;
mod load;

impl serde::Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn app_path(handle: &AppHandle) -> PathBuf {
    // get app_dir
    // if success, print the path
    // if failed, expect the error
    let app_path = handle
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app path");

    println!("app path: {:?}", app_path);

    app_path
}

fn db_path(mut base: PathBuf) -> String {
    base.push("data.db");

    format!("sqlite://{}", base.to_str().expect("Failed to get db path"))
}

async fn migrate_database(pool: &SqlitePool) -> Result<(), CustomError> {
    let migrator = sqlx::migrate!("./migrations");
    migrator
        .run(pool)
        .await
        .with_context(|| "Failed to run migrations")?;
    Ok(())
}

async fn initialize_sqlite(handle: AppHandle) -> Result<SqlitePool, CustomError> {
    let data_path = app_path(&handle);
    let db_path = db_path(data_path.clone());

    // create data dir
    create_dir_all(&data_path)
        .with_context(|| format!("Failed to create data dir at {:?}", data_path))?;

    let db_exists = Sqlite::database_exists(&db_path)
        .await
        .with_context(|| format!("Failed to check if database exists at {}", db_path))?;

    // create the sqlite database if it does not exist
    if !db_exists {
        Sqlite::create_database(&db_path)
            .await
            .with_context(|| format!("Failed to create database at {}", db_path))?;

        // print the path
        println!("sqlite database created at {}", db_path);
    }

    let pool = get_sqlite_pool(db_path.clone())
        .await
        .with_context(|| format!("Failed to get sqlite pool at {}", db_path))?;

    // run migrations
    if !db_exists {
        migrate_database(&pool)
            .await
            .with_context(|| "Failed to run migrations")?;
    }

    Ok(pool)
}

async fn get_sqlite_pool(path: String) -> Result<SqlitePool, CustomError> {
    let pool = SqlitePool::connect(&path)
        .await
        .context("Failed to connect to sqlite")?;
    Ok(pool)
}

#[tauri::command]
async fn load_csv(pool: State<'_, SqlitePool>, path: &str) -> Result<(), CustomError> {
    let file = load::file_open(path).context("Failed to open file")?;
    let records = load::csv_parse(file).context("Failed to parse CSV")?;

    // initialize the desc table with the records
    load::initialize_desc_table_by_records(&pool, records).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
struct Description {
    pub id: i32,
    pub source_id: String,
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub actual_start_at: String,
}

#[tauri::command]
async fn get_description_by_source_id(
    pool: State<'_, SqlitePool>,
    source_id: String,
) -> Result<String, CustomError> {
    get_description_by_source_id_infra(pool, &source_id)
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Failed to get description by source_id: {}", e)))
        .map(|desc| serde_json::to_string(&desc).unwrap())
}

async fn get_description_by_source_id_infra(
    pool: State<'_, SqlitePool>,
    source_id: &str,
) -> Result<Description, sqlx::Error> {
    // get pool from the state
    let p = pool.clone();
    // get description by source_id from the sqlite database
    let desc = sqlx::query_as::<_, Description>(
        r#"
        SELECT * FROM desc WHERE source_id = ?
        "#,
    )
    .bind(source_id)
    .fetch_one(&*p)
    .await?;

    Ok(desc)
}

async fn close_sqlite_pool(pool: Pool<Sqlite>) {
    pool.close().await;
}

fn main() {
    use tauri::async_runtime::block_on;

    dotenv().expect("Failed to load .env file");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_csv,
            get_description_by_source_id
        ])
        .setup(|app| {
            let pool = block_on(initialize_sqlite(app.handle()))
                .expect("Failed to initialize sqlite pool");
            app.manage(pool);
            Ok(())
        })
        .build(tauri::generate_context!()).expect("error while building tauri application")
        .run( |_app_handle, event| {
            if let RunEvent::ExitRequested { ref api, .. } = event {
                api.prevent_exit();
            }
            if let RunEvent::WindowEvent { label, event, .. } = event {
                if let tauri::WindowEvent::Destroyed = event {
                    if label == "main" {
                        let pool = _app_handle.state::<Pool<Sqlite>>();
                        tauri::async_runtime::block_on(close_sqlite_pool(pool.inner().clone()));
                        _app_handle.exit(0);
                    }
                }
            }
        })
}
