//! Práctica 04: Middleware
//!
//! Implementar middleware de logging y autenticación.

use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde::Serialize;
use std::time::Instant;
use tower_http::trace::TraceLayer;

// =============================================================================
// MIDDLEWARE DE LOGGING
// =============================================================================

/// Middleware que registra cada request con tiempo de respuesta
async fn logging_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();

    // Log de entrada
    tracing::info!("➡️  {} {}", method, uri);

    // Ejecutar el handler
    let response = next.run(request).await;

    // Log de salida con duración
    let duration = start.elapsed();
    let status = response.status();
    
    if status.is_success() {
        tracing::info!(
            "⬅️  {} {} -> {} ({:?})",
            method, uri, status.as_u16(), duration
        );
    } else {
        tracing::warn!(
            "⚠️  {} {} -> {} ({:?})",
            method, uri, status.as_u16(), duration
        );
    }

    response
}

// =============================================================================
// MIDDLEWARE DE AUTENTICACIÓN
// =============================================================================

/// Middleware que verifica el header Authorization
async fn auth_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    // Obtener el header Authorization
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(token) if token.starts_with("Bearer ") => {
            let token_value = &token[7..]; // Quitar "Bearer "
            
            // Validar token (simplificado - en producción usar JWT)
            if token_value == "mi-token-secreto" {
                tracing::info!("🔓 Token válido");
                Ok(next.run(request).await)
            } else {
                tracing::warn!("🔒 Token inválido: {}", token_value);
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        Some(_) => {
            tracing::warn!("🔒 Formato de Authorization incorrecto");
            Err(StatusCode::UNAUTHORIZED)
        }
        None => {
            tracing::warn!("🔒 Sin header Authorization");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

// =============================================================================
// MIDDLEWARE DE REQUEST ID
// =============================================================================

/// Middleware que añade un ID único a cada request
async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    // Generar ID único (simplificado)
    let request_id = format!("req-{}", rand_id());
    
    // Añadir a los headers del request
    request.headers_mut().insert(
        "X-Request-Id",
        request_id.parse().unwrap(),
    );
    
    tracing::info!("📋 Request ID: {}", request_id);
    
    let mut response = next.run(request).await;
    
    // Añadir a los headers de la response
    response.headers_mut().insert(
        "X-Request-Id",
        request_id.parse().unwrap(),
    );
    
    response
}

/// Generar ID simple (en producción usar UUID)
fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}

// =============================================================================
// HANDLERS
// =============================================================================

#[derive(Serialize)]
struct InfoResponse {
    mensaje: String,
    version: String,
}

/// Endpoint público (sin auth)
async fn publico() -> impl IntoResponse {
    Json(InfoResponse {
        mensaje: "Este endpoint es público".to_string(),
        version: "1.0.0".to_string(),
    })
}

/// Endpoint que requiere autenticación
async fn privado() -> impl IntoResponse {
    Json(InfoResponse {
        mensaje: "¡Acceso autorizado al área privada!".to_string(),
        version: "1.0.0".to_string(),
    })
}

/// Endpoint de administración
async fn admin() -> impl IntoResponse {
    Json(InfoResponse {
        mensaje: "Panel de administración".to_string(),
        version: "1.0.0".to_string(),
    })
}

/// Health check
async fn health() -> &'static str {
    "OK"
}

// =============================================================================
// ROUTER
// =============================================================================

pub fn crear_app() -> Router {
    // Rutas públicas
    let rutas_publicas = Router::new()
        .route("/", get(publico))
        .route("/health", get(health));

    // Rutas protegidas (requieren auth)
    let rutas_protegidas = Router::new()
        .route("/privado", get(privado))
        .route("/admin", get(admin))
        .layer(middleware::from_fn(auth_middleware));

    // Combinar y aplicar middleware global
    Router::new()
        .merge(rutas_publicas)
        .merge(rutas_protegidas)
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn(logging_middleware))
        .layer(TraceLayer::new_for_http())
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    // Inicializar tracing/logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = crear_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("No se pudo iniciar el servidor");

    tracing::info!("🚀 API con Middleware");
    tracing::info!("   http://localhost:3000");
    tracing::info!("");
    tracing::info!("📝 Endpoints públicos:");
    tracing::info!("   GET /        - Info pública");
    tracing::info!("   GET /health  - Health check");
    tracing::info!("");
    tracing::info!("🔒 Endpoints protegidos (requieren Authorization):");
    tracing::info!("   GET /privado - Área privada");
    tracing::info!("   GET /admin   - Administración");
    tracing::info!("");
    tracing::info!("💡 Token válido: Bearer mi-token-secreto");

    axum::serve(listener, app).await.unwrap();
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_ruta_publica() {
        let app = crear_app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_ruta_protegida_sin_token() {
        let app = crear_app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/privado")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_ruta_protegida_con_token() {
        let app = crear_app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/privado")
                    .header("Authorization", "Bearer mi-token-secreto")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_token_invalido() {
        let app = crear_app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/privado")
                    .header("Authorization", "Bearer token-incorrecto")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
