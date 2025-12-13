//! Definición de rutas de la API

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::SqlitePool;

use crate::handlers;

/// Crear router de la API
pub fn crear_rutas() -> Router<SqlitePool> {
    Router::new()
        .route("/tareas", get(handlers::listar).post(handlers::crear))
        .route("/tareas/estadisticas", get(handlers::estadisticas))
        .route(
            "/tareas/{id}",
            get(handlers::obtener)
                .put(handlers::actualizar)
                .delete(handlers::eliminar),
        )
}
