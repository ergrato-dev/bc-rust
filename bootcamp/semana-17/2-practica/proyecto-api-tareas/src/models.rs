//! Modelos de datos

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Tarea almacenada en la base de datos
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tarea {
    pub id: i64,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub completada: bool,
    pub creada_en: String,
    pub actualizada_en: String,
}

/// DTO para crear una tarea
#[derive(Debug, Deserialize)]
pub struct CrearTarea {
    pub titulo: String,
    pub descripcion: Option<String>,
}

/// DTO para actualizar una tarea
#[derive(Debug, Deserialize)]
pub struct ActualizarTarea {
    pub titulo: Option<String>,
    pub descripcion: Option<String>,
    pub completada: Option<bool>,
}

/// Filtros de consulta
#[derive(Debug, Deserialize, Default)]
pub struct FiltroTareas {
    pub completada: Option<bool>,
    pub limite: Option<i64>,
    pub offset: Option<i64>,
}

/// Estadísticas de tareas
#[derive(Debug, Serialize)]
pub struct EstadisticasTareas {
    pub total: i64,
    pub completadas: i64,
    pub pendientes: i64,
}
