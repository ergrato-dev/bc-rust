//! Modelos de datos

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// Tarea almacenada en la base de datos
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Tarea {
    /// ID único de la tarea
    #[schema(example = 1)]
    pub id: i64,
    /// Título de la tarea
    #[schema(example = "Aprender Rust")]
    pub titulo: String,
    /// Descripción opcional
    #[schema(example = "Completar el bootcamp de 17 semanas")]
    pub descripcion: Option<String>,
    /// Estado de completitud
    #[schema(example = false)]
    pub completada: bool,
    /// Fecha de creación
    #[schema(example = "2025-01-15 10:30:00")]
    pub creada_en: String,
    /// Fecha de última actualización
    #[schema(example = "2025-01-15 12:00:00")]
    pub actualizada_en: String,
}

/// DTO para crear una tarea
#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearTarea {
    /// Título de la tarea (requerido)
    #[schema(example = "Aprender Rust")]
    pub titulo: String,
    /// Descripción opcional de la tarea
    #[schema(example = "Completar el bootcamp")]
    pub descripcion: Option<String>,
}

/// DTO para actualizar una tarea
#[derive(Debug, Deserialize, ToSchema)]
pub struct ActualizarTarea {
    /// Nuevo título (opcional)
    #[schema(example = "Dominar Rust")]
    pub titulo: Option<String>,
    /// Nueva descripción (opcional)
    #[schema(example = "Ahora soy un experto")]
    pub descripcion: Option<String>,
    /// Nuevo estado de completitud (opcional)
    #[schema(example = true)]
    pub completada: Option<bool>,
}

/// Filtros de consulta
#[derive(Debug, Deserialize, Default, ToSchema)]
pub struct FiltroTareas {
    /// Filtrar por estado de completitud
    #[schema(example = false)]
    pub completada: Option<bool>,
    /// Límite de resultados
    #[schema(example = 10)]
    pub limite: Option<i64>,
    /// Offset para paginación
    #[schema(example = 0)]
    pub offset: Option<i64>,
}

/// Estadísticas de tareas
#[derive(Debug, Serialize, ToSchema)]
pub struct EstadisticasTareas {
    /// Total de tareas
    #[schema(example = 100)]
    pub total: i64,
    /// Tareas completadas
    #[schema(example = 75)]
    pub completadas: i64,
    /// Tareas pendientes
    #[schema(example = 25)]
    pub pendientes: i64,
}

/// Respuesta de error de la API
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Mensaje de error
    #[schema(example = "Tarea no encontrada")]
    pub error: String,
    /// Código HTTP
    #[schema(example = 404)]
    pub codigo: u16,
}
