use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};
use tauri::State;

use crate::custom_error::CustomError;
use crate::db::db_error::DbError;

#[derive(Debug,Serialize, Deserialize, FromRow)]
struct Description {
    id: i32,
    source_id: String,
    title: String,
    description: String,
    published_at: String,
    actual_start_at: String,
}

impl Description {
    // accessor
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn published_at(&self) -> &str {
        &self.published_at
    }

    pub fn actual_start_at(&self) -> &str {
        &self.actual_start_at
    }
}

async fn get_description_by_source_id(
    pool: State<'_, SqlitePool>,
    source_id: &str,
) -> Result<Description, CustomError> {
    let p = pool.clone();

    let desc = sqlx::query_as::<_, Description>(
        r#"
        SELECT * FROM desc WHERE source_id = ?
        "#,
    ).bind(source_id).fetch_one(&*p).await.map_err(|e| CustomError::DbError(DbError::Query(e)))?;

    Ok(desc)
}

#[derive(Debug, Serialize, Deserialize)]
struct PageResult {
    page: Vec<Description>,
    has_prev_page: bool,
    has_next_page: bool,
}

// pagination with seek method
async fn get_description_page(
    pool: State<'_, SqlitePool>,
    page_size: i32,
    last_id: i32,
) -> Result<PageResult, CustomError> {
    let p = pool.clone();
    let rows = sqlx::query(
        r#"
        SELECT * FROM desc WHERE id > ? ORDER BY id ASC LIMIT ?
        "#,
    ).bind(last_id).bind(page_size+1).fetch_all(&*p).await.map_err(|e| CustomError::DbError(DbError::Query(e)))?;

    let mut items = Vec::new();
    for row in rows {
        items.push(Description {
            id: row.get("id"),
            source_id: row.get("source_id"),
            title: row.get("title"),
            description: row.get("description"),
            published_at: row.get("published_at"),
            actual_start_at: row.get("actual_start_at"),
        });
    }

    // TODO: implement has_prev_page and has_next_page

    Ok(PageResult {
        page: items,
        has_prev_page: false,
        has_next_page: false,
    })
}
