//! Práctica 03: Validación de Requests
//!
//! Implementar validación de datos de entrada con errores descriptivos.

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response, Json},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

// =============================================================================
// MODELOS
// =============================================================================

/// Datos para registrar un usuario
#[derive(Debug, Deserialize)]
pub struct RegistroUsuario {
    pub nombre: String,
    pub email: String,
    pub edad: u8,
    pub password: String,
}

/// Usuario registrado (sin password)
#[derive(Debug, Serialize)]
pub struct Usuario {
    pub id: u64,
    pub nombre: String,
    pub email: String,
    pub edad: u8,
}

/// Respuesta de error estructurada
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub codigo: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detalles: Option<Vec<String>>,
}

// =============================================================================
// ERROR PERSONALIZADO
// =============================================================================

/// Errores de la API
pub enum ApiError {
    /// Error de validación con detalles
    Validacion(Vec<String>),
    /// JSON malformado
    JsonInvalido(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, detalles) = match self {
            ApiError::Validacion(errores) => (
                StatusCode::BAD_REQUEST,
                "Error de validación".to_string(),
                Some(errores),
            ),
            ApiError::JsonInvalido(msg) => (
                StatusCode::BAD_REQUEST,
                format!("JSON inválido: {}", msg),
                None,
            ),
        };

        let body = ErrorResponse {
            error,
            codigo: status.as_u16(),
            detalles,
        };

        (status, Json(body)).into_response()
    }
}

// =============================================================================
// VALIDACIÓN
// =============================================================================

/// Validar datos de registro
fn validar_registro(datos: &RegistroUsuario) -> Result<(), Vec<String>> {
    let mut errores = Vec::new();

    // Validar nombre
    if datos.nombre.trim().is_empty() {
        errores.push("El nombre es requerido".to_string());
    } else if datos.nombre.len() < 2 {
        errores.push("El nombre debe tener al menos 2 caracteres".to_string());
    } else if datos.nombre.len() > 100 {
        errores.push("El nombre no puede exceder 100 caracteres".to_string());
    }

    // Validar email
    if datos.email.trim().is_empty() {
        errores.push("El email es requerido".to_string());
    } else if !datos.email.contains('@') || !datos.email.contains('.') {
        errores.push("El email no tiene un formato válido".to_string());
    }

    // Validar edad
    if datos.edad < 18 {
        errores.push("Debes ser mayor de 18 años".to_string());
    } else if datos.edad > 120 {
        errores.push("Edad no válida".to_string());
    }

    // Validar password
    if datos.password.len() < 8 {
        errores.push("La contraseña debe tener al menos 8 caracteres".to_string());
    }
    if !datos.password.chars().any(|c| c.is_uppercase()) {
        errores.push("La contraseña debe contener al menos una mayúscula".to_string());
    }
    if !datos.password.chars().any(|c| c.is_numeric()) {
        errores.push("La contraseña debe contener al menos un número".to_string());
    }

    if errores.is_empty() {
        Ok(())
    } else {
        Err(errores)
    }
}

// =============================================================================
// HANDLERS
// =============================================================================

/// POST /registro - Registrar nuevo usuario con validación
async fn registrar(
    payload: Result<Json<RegistroUsuario>, JsonRejection>,
) -> Result<(StatusCode, Json<Usuario>), ApiError> {
    // Primero verificar que el JSON sea válido
    let Json(datos) = payload.map_err(|e| ApiError::JsonInvalido(e.to_string()))?;

    // Validar los datos
    validar_registro(&datos).map_err(ApiError::Validacion)?;

    // Crear usuario (simulado)
    let usuario = Usuario {
        id: 1,
        nombre: datos.nombre,
        email: datos.email,
        edad: datos.edad,
    };

    Ok((StatusCode::CREATED, Json(usuario)))
}

/// POST /contacto - Formulario de contacto simple
async fn contacto(
    payload: Result<Json<ContactoForm>, JsonRejection>,
) -> Result<Json<ContactoResponse>, ApiError> {
    let Json(datos) = payload.map_err(|e| ApiError::JsonInvalido(e.to_string()))?;

    let mut errores = Vec::new();

    if datos.nombre.trim().is_empty() {
        errores.push("El nombre es requerido".to_string());
    }
    if datos.mensaje.trim().is_empty() {
        errores.push("El mensaje es requerido".to_string());
    } else if datos.mensaje.len() < 10 {
        errores.push("El mensaje debe tener al menos 10 caracteres".to_string());
    }

    if !errores.is_empty() {
        return Err(ApiError::Validacion(errores));
    }

    Ok(Json(ContactoResponse {
        mensaje: "Mensaje recibido correctamente".to_string(),
        id_ticket: "TKT-001".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct ContactoForm {
    pub nombre: String,
    pub mensaje: String,
}

#[derive(Debug, Serialize)]
pub struct ContactoResponse {
    pub mensaje: String,
    pub id_ticket: String,
}

// =============================================================================
// ROUTER Y MAIN
// =============================================================================

pub fn crear_app() -> Router {
    Router::new()
        .route("/registro", post(registrar))
        .route("/contacto", post(contacto))
}

#[tokio::main]
async fn main() {
    let app = crear_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("No se pudo iniciar el servidor");

    println!("🚀 API con Validación");
    println!("   http://localhost:3000");
    println!();
    println!("📝 Endpoints:");
    println!("   POST /registro - Registrar usuario (validación completa)");
    println!("   POST /contacto - Formulario de contacto");
    println!();
    println!("💡 Prueba enviar datos inválidos para ver los errores");

    axum::serve(listener, app).await.unwrap();
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validacion_exitosa() {
        let datos = RegistroUsuario {
            nombre: "Ana García".to_string(),
            email: "ana@ejemplo.com".to_string(),
            edad: 25,
            password: "MiPassword123".to_string(),
        };
        assert!(validar_registro(&datos).is_ok());
    }

    #[test]
    fn test_nombre_vacio() {
        let datos = RegistroUsuario {
            nombre: "".to_string(),
            email: "ana@ejemplo.com".to_string(),
            edad: 25,
            password: "MiPassword123".to_string(),
        };
        let result = validar_registro(&datos);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("nombre")));
    }

    #[test]
    fn test_email_invalido() {
        let datos = RegistroUsuario {
            nombre: "Ana".to_string(),
            email: "email-sin-arroba".to_string(),
            edad: 25,
            password: "MiPassword123".to_string(),
        };
        let result = validar_registro(&datos);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("email")));
    }

    #[test]
    fn test_edad_menor() {
        let datos = RegistroUsuario {
            nombre: "Ana".to_string(),
            email: "ana@ejemplo.com".to_string(),
            edad: 15,
            password: "MiPassword123".to_string(),
        };
        let result = validar_registro(&datos);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("18")));
    }

    #[test]
    fn test_password_debil() {
        let datos = RegistroUsuario {
            nombre: "Ana".to_string(),
            email: "ana@ejemplo.com".to_string(),
            edad: 25,
            password: "abc".to_string(),
        };
        let result = validar_registro(&datos);
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(errores.len() >= 2); // Al menos 2 errores de password
    }

    #[test]
    fn test_multiples_errores() {
        let datos = RegistroUsuario {
            nombre: "".to_string(),
            email: "invalido".to_string(),
            edad: 10,
            password: "123".to_string(),
        };
        let result = validar_registro(&datos);
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(errores.len() >= 4);
    }
}
