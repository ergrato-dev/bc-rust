//! Configuración de base de datos SQLite

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

/// Pool de conexiones a SQLite
pub type DbPool = SqlitePool;

/// Crear pool de conexiones a SQLite
pub async fn crear_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:tareas.db?mode=rwc".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Crear tabla si no existe
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

    // Crear índice para búsquedas por estado
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
pub async fn crear_pool_test() -> Result<DbPool, sqlx::Error> {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    crear_pool().await
}
