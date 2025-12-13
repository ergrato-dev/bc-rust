//! Handlers de la API de Tareas

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use sqlx::SqlitePool;

use crate::error::{ApiError, Result};
use crate::models::{ActualizarTarea, CrearTarea, EstadisticasTareas, FiltroTareas, Tarea};

/// GET /tareas - Listar todas las tareas
pub async fn listar(
    State(pool): State<SqlitePool>,
    Query(filtro): Query<FiltroTareas>,
) -> Result<Json<Vec<Tarea>>> {
    let limite = filtro.limite.unwrap_or(100);
    let offset = filtro.offset.unwrap_or(0);

    let tareas = match filtro.completada {
        Some(completada) => {
            sqlx::query_as::<_, Tarea>(
                "SELECT * FROM tareas WHERE completada = ? ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(completada)
            .bind(limite)
            .bind(offset)
            .fetch_all(&pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, Tarea>(
                "SELECT * FROM tareas ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(limite)
            .bind(offset)
            .fetch_all(&pool)
            .await?
        }
    };

    Ok(Json(tareas))
}

/// GET /tareas/:id - Obtener una tarea
pub async fn obtener(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Tarea>> {
    let tarea = sqlx::query_as::<_, Tarea>("SELECT * FROM tareas WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Tarea {} no encontrada", id)))?;

    Ok(Json(tarea))
}

/// POST /tareas - Crear nueva tarea
pub async fn crear(
    State(pool): State<SqlitePool>,
    Json(datos): Json<CrearTarea>,
) -> Result<(StatusCode, Json<Tarea>)> {
    // Validación
    if datos.titulo.trim().is_empty() {
        return Err(ApiError::Validacion("El título es requerido".into()));
    }

    if datos.titulo.len() > 200 {
        return Err(ApiError::Validacion(
            "El título no puede exceder 200 caracteres".into(),
        ));
    }

    let resultado = sqlx::query("INSERT INTO tareas (titulo, descripcion) VALUES (?, ?)")
        .bind(&datos.titulo)
        .bind(&datos.descripcion)
        .execute(&pool)
        .await?;

    let id = resultado.last_insert_rowid();

    let tarea = sqlx::query_as::<_, Tarea>("SELECT * FROM tareas WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(tarea)))
}

/// PUT /tareas/:id - Actualizar tarea
pub async fn actualizar(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(datos): Json<ActualizarTarea>,
) -> Result<Json<Tarea>> {
    // Verificar que existe
    let existe = sqlx::query("SELECT id FROM tareas WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    if existe.is_none() {
        return Err(ApiError::NotFound(format!("Tarea {} no encontrada", id)));
    }

    // Actualizar campos proporcionados
    if let Some(titulo) = &datos.titulo {
        if titulo.trim().is_empty() {
            return Err(ApiError::Validacion("El título no puede estar vacío".into()));
        }
        sqlx::query(
            "UPDATE tareas SET titulo = ?, actualizada_en = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(titulo)
        .bind(id)
        .execute(&pool)
        .await?;
    }

    if let Some(descripcion) = &datos.descripcion {
        sqlx::query(
            "UPDATE tareas SET descripcion = ?, actualizada_en = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(descripcion)
        .bind(id)
        .execute(&pool)
        .await?;
    }

    if let Some(completada) = datos.completada {
        sqlx::query(
            "UPDATE tareas SET completada = ?, actualizada_en = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(completada)
        .bind(id)
        .execute(&pool)
        .await?;
    }

    // Obtener tarea actualizada
    let tarea = sqlx::query_as::<_, Tarea>("SELECT * FROM tareas WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(tarea))
}

/// DELETE /tareas/:id - Eliminar tarea
pub async fn eliminar(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let resultado = sqlx::query("DELETE FROM tareas WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if resultado.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("Tarea {} no encontrada", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// GET /tareas/estadisticas - Obtener estadísticas
pub async fn estadisticas(State(pool): State<SqlitePool>) -> Result<Json<EstadisticasTareas>> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tareas")
        .fetch_one(&pool)
        .await?;

    let completadas: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM tareas WHERE completada = TRUE")
            .fetch_one(&pool)
            .await?;

    Ok(Json(EstadisticasTareas {
        total: total.0,
        completadas: completadas.0,
        pendientes: total.0 - completadas.0,
    }))
}
