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
// MODELS
// =============================================================================

/// User registration data
#[derive(Debug, Deserialize)]
pub struct UserRegistration {
    pub name: String,
    pub email: String,
    pub age: u8,
    pub password: String,
}

/// Registered user (without password)
#[derive(Debug, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub age: u8,
}

/// Structured error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

// =============================================================================
// CUSTOM ERROR
// =============================================================================

/// API errors
pub enum ApiError {
    /// Validation error with details
    Validation(Vec<String>),
    /// Malformed JSON
    InvalidJson(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, details) = match self {
            ApiError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_string(),
                Some(errors),
            ),
            ApiError::InvalidJson(msg) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON: {}", msg),
                None,
            ),
        };

        let body = ErrorResponse {
            error,
            code: status.as_u16(),
            details,
        };

        (status, Json(body)).into_response()
    }
}

// =============================================================================
// VALIDATION
// =============================================================================

/// Validate registration data
fn validate_registration(data: &UserRegistration) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Validate name
    if data.name.trim().is_empty() {
        errors.push("El nombre es requerido".to_string());
    } else if data.name.len() < 2 {
        errors.push("El nombre debe tener al menos 2 caracteres".to_string());
    } else if data.name.len() > 100 {
        errors.push("El nombre no puede exceder 100 caracteres".to_string());
    }

    // Validate email
    if data.email.trim().is_empty() {
        errors.push("El email es requerido".to_string());
    } else if !data.email.contains('@') || !data.email.contains('.') {
        errors.push("El email no tiene un formato válido".to_string());
    }

    // Validate age
    if data.age < 18 {
        errors.push("Debes ser mayor de 18 años".to_string());
    } else if data.age > 120 {
        errors.push("Edad no válida".to_string());
    }

    // Validate password
    if data.password.len() < 8 {
        errors.push("La contraseña debe tener al menos 8 caracteres".to_string());
    }
    if !data.password.chars().any(|c| c.is_uppercase()) {
        errors.push("La contraseña debe contener al menos una mayúscula".to_string());
    }
    if !data.password.chars().any(|c| c.is_numeric()) {
        errors.push("La contraseña debe contener al menos un número".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// =============================================================================
// HANDLERS
// =============================================================================

/// POST /register - Register new user with validation
async fn register(
    payload: Result<Json<UserRegistration>, JsonRejection>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    // First verify JSON is valid
    let Json(data) = payload.map_err(|e| ApiError::InvalidJson(e.to_string()))?;

    // Validate data
    validate_registration(&data).map_err(ApiError::Validation)?;

    // Create user (simulated)
    let user = User {
        id: 1,
        name: data.name,
        email: data.email,
        age: data.age,
    };

    Ok((StatusCode::CREATED, Json(user)))
}

/// POST /contact - Simple contact form
async fn contact(
    payload: Result<Json<ContactForm>, JsonRejection>,
) -> Result<Json<ContactResponse>, ApiError> {
    let Json(data) = payload.map_err(|e| ApiError::InvalidJson(e.to_string()))?;

    let mut errors = Vec::new();

    if data.name.trim().is_empty() {
        errors.push("El nombre es requerido".to_string());
    }
    if data.message.trim().is_empty() {
        errors.push("El mensaje es requerido".to_string());
    } else if data.message.len() < 10 {
        errors.push("El mensaje debe tener al menos 10 caracteres".to_string());
    }

    if !errors.is_empty() {
        return Err(ApiError::Validation(errors));
    }

    Ok(Json(ContactResponse {
        message: "Mensaje recibido correctamente".to_string(),
        ticket_id: "TKT-001".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub message: String,
    pub ticket_id: String,
}

// =============================================================================
// ROUTER AND MAIN
// =============================================================================

pub fn create_app() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/contact", post(contact))
}

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not start server");

    println!("🚀 API with Validation");
    println!("   http://localhost:3000");
    println!();
    println!("📝 Endpoints:");
    println!("   POST /register - Register user (full validation)");
    println!("   POST /contact  - Contact form");
    println!();
    println!("💡 Try sending invalid data to see errors");

    axum::serve(listener, app).await.unwrap();
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_success() {
        let data = UserRegistration {
            name: "Ana García".to_string(),
            email: "ana@ejemplo.com".to_string(),
            age: 25,
            password: "MiPassword123".to_string(),
        };
        assert!(validate_registration(&data).is_ok());
    }

    #[test]
    fn test_empty_name() {
        let data = UserRegistration {
            name: "".to_string(),
            email: "ana@ejemplo.com".to_string(),
            age: 25,
            password: "MiPassword123".to_string(),
        };
        let result = validate_registration(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("nombre")));
    }

    #[test]
    fn test_invalid_email() {
        let data = UserRegistration {
            name: "Ana".to_string(),
            email: "email-sin-arroba".to_string(),
            age: 25,
            password: "MiPassword123".to_string(),
        };
        let result = validate_registration(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("email")));
    }

    #[test]
    fn test_underage() {
        let data = UserRegistration {
            name: "Ana".to_string(),
            email: "ana@ejemplo.com".to_string(),
            age: 15,
            password: "MiPassword123".to_string(),
        };
        let result = validate_registration(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("18")));
    }

    #[test]
    fn test_weak_password() {
        let data = UserRegistration {
            name: "Ana".to_string(),
            email: "ana@ejemplo.com".to_string(),
            age: 25,
            password: "abc".to_string(),
        };
        let result = validate_registration(&data);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2); // At least 2 password errors
    }

    #[test]
    fn test_multiple_errors() {
        let data = UserRegistration {
            name: "".to_string(),
            email: "invalido".to_string(),
            age: 10,
            password: "123".to_string(),
        };
        let result = validate_registration(&data);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 4);
    }
}
