use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, sqlx::FromRow)]
struct History {
    pub id: i32,
    pub path: String,
    pub count: i32,
    pub loaded_at: String,
}

async fn get_load_history(pool: State<'_,SqlitePool>) -> Result<Vec<History>,Err()>{
    let p = pool.clone();
    let history = sqlx::query_as::<_, History>(
        r#"
        SELECT * FROM load_history ORDER BY id DESC
        "#,
    ).fetch_all(&*p).await?;

    Ok(history)
}
