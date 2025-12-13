//! Handlers de la API de Tareas

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use sqlx::SqlitePool;

use crate::error::{ApiError, Result};
use crate::models::{ActualizarTarea, CrearTarea, EstadisticasTareas, FiltroTareas, Tarea};

/// Listar todas las tareas
///
/// Obtiene una lista de tareas con soporte para filtros y paginación.
#[utoipa::path(
    get,
    path = "/tareas",
    params(
        ("completada" = Option<bool>, Query, description = "Filtrar por estado de completitud"),
        ("limite" = Option<i64>, Query, description = "Límite de resultados (default: 100)"),
        ("offset" = Option<i64>, Query, description = "Offset para paginación (default: 0)")
    ),
    responses(
        (status = 200, description = "Lista de tareas", body = Vec<Tarea>),
        (status = 500, description = "Error interno", body = crate::models::ErrorResponse)
    ),
    tag = "Tareas"
)]
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

/// Obtener una tarea por ID
///
/// Retorna los detalles de una tarea específica.
#[utoipa::path(
    get,
    path = "/tareas/{id}",
    params(
        ("id" = i64, Path, description = "ID de la tarea")
    ),
    responses(
        (status = 200, description = "Tarea encontrada", body = Tarea),
        (status = 404, description = "Tarea no encontrada", body = crate::models::ErrorResponse)
    ),
    tag = "Tareas"
)]
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

/// Crear una nueva tarea
///
/// Crea una tarea con el título proporcionado.
#[utoipa::path(
    post,
    path = "/tareas",
    request_body = CrearTarea,
    responses(
        (status = 201, description = "Tarea creada exitosamente", body = Tarea),
        (status = 400, description = "Error de validación", body = crate::models::ErrorResponse)
    ),
    tag = "Tareas"
)]
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

/// Actualizar una tarea existente
///
/// Actualiza uno o más campos de una tarea.
#[utoipa::path(
    put,
    path = "/tareas/{id}",
    params(
        ("id" = i64, Path, description = "ID de la tarea a actualizar")
    ),
    request_body = ActualizarTarea,
    responses(
        (status = 200, description = "Tarea actualizada", body = Tarea),
        (status = 400, description = "Error de validación", body = crate::models::ErrorResponse),
        (status = 404, description = "Tarea no encontrada", body = crate::models::ErrorResponse)
    ),
    tag = "Tareas"
)]
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

/// Eliminar una tarea
///
/// Elimina permanentemente una tarea de la base de datos.
#[utoipa::path(
    delete,
    path = "/tareas/{id}",
    params(
        ("id" = i64, Path, description = "ID de la tarea a eliminar")
    ),
    responses(
        (status = 204, description = "Tarea eliminada exitosamente"),
        (status = 404, description = "Tarea no encontrada", body = crate::models::ErrorResponse)
    ),
    tag = "Tareas"
)]
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

/// Obtener estadísticas de tareas
///
/// Retorna el conteo total, completadas y pendientes.
#[utoipa::path(
    get,
    path = "/tareas/estadisticas",
    responses(
        (status = 200, description = "Estadísticas de tareas", body = EstadisticasTareas)
    ),
    tag = "Estadísticas"
)]
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
