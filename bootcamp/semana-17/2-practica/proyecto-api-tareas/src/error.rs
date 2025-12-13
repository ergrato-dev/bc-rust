//! Manejo de errores de la API

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Errores de la API
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("No encontrado: {0}")]
    NotFound(String),

    #[error("Error de validación: {0}")]
    Validacion(String),

    #[error("Error de base de datos")]
    Database(#[from] sqlx::Error),

    #[error("Error interno: {0}")]
    Interno(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, mensaje) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::Validacion(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::Database(e) => {
                tracing::error!("Error de base de datos: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error de base de datos".to_string(),
                )
            }
            ApiError::Interno(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = json!({
            "error": mensaje,
            "codigo": status.as_u16()
        });

        (status, Json(body)).into_response()
    }
}

/// Tipo Result para handlers
pub type Result<T> = std::result::Result<T, ApiError>;
