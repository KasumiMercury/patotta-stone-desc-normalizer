use sqlx::SqlitePool;
use tauri::State;

use crate::custom_error::CustomError;
use crate::db::db_error::DbError;

#[derive((Debug, FromRow))]
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

async fn get_description_page() -> Result<Vec<Description>,CustomError>{
    let page: Vec<Description> = Vec::new();
    // TODO:Implement pagination
    Ok(page)
}
