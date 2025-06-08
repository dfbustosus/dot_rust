use crate::config::AppConfig;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

// Using SQLite for the database
pub type DbPool = Pool<Sqlite>;

pub async fn init_db_pool(config: &AppConfig) -> Result<DbPool, sqlx::Error> {
    // Create a SQLite connection pool
    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database.url)
        .await?;
            
    // Initialize the SQLite database with our schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&sqlite_pool)
    .await?;
    
    Ok(sqlite_pool)
}
