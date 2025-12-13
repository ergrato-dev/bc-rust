//! SQLite Database Configuration

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

/// SQLite connection pool
pub type DbPool = SqlitePool;

/// Create SQLite connection pool
pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:tasks.db?mode=rwc".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tareas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            descripcion TEXT,
            completada BOOLEAN NOT NULL DEFAULT FALSE,
            creada_en DATETIME DEFAULT CURRENT_TIMESTAMP,
            actualizada_en DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Create index for status searches
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_tareas_completada 
        ON tareas(completada)
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

#[cfg(test)]
pub async fn create_test_pool() -> Result<DbPool, sqlx::Error> {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    create_pool().await
}
