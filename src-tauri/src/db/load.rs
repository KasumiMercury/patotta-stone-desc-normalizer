use sqlx::SqlitePool;
use tauri::State;

use crate::custom_error::CustomError;
use crate::db::db_error::DbError;

#[derive(Debug, sqlx::FromRow)]
struct History {
    id: i32,
    path: String,
    count: i32,
    loaded_at: String,
}

impl History {
    // accessors
    pub fn id(&self) -> i32{
        self.id
    }

    pub fn path(&self) -> &str{
        &self.path
    }

    pub fn count(&self) -> i32{
        self.count
    }

    pub fn loaded_at(&self) -> &str{
        &self.loaded_at
    }
}

async fn get_load_history(pool: State<'_,SqlitePool>) -> Result<Vec<History>,CustomError>{
    let p = pool.clone();
    let history = sqlx::query_as::<_, History>(
        r#"
        SELECT * FROM load_history ORDER BY id DESC
        "#,
    ).fetch_all(&*p).await.map_err(|e| CustomError::DbError(DbError::Query(e)))?;

    Ok(history)
}
