//! Práctica 01: Hello Axum
//!
//! Tu primer servidor HTTP con Axum.
//!
//! # Objetivos
//! - Crear un servidor básico
//! - Definir rutas simples
//! - Usar parámetros de ruta

use axum::{
    extract::Path,
    routing::get,
    Router,
};

/// Root route handler
async fn root() -> &'static str {
    "¡Bienvenido a mi primera API con Axum! 🦀"
}

/// Handler de salud del servicio
async fn health() -> &'static str {
    "OK"
}

/// Route parameter handler
///
/// # Example
/// GET /greet/Ana -> "¡Hola, Ana! 👋"
async fn greet(Path(name): Path<String>) -> String {
    format!("¡Hola, {}! 👋", name)
}

/// Handler que retorna información del API
async fn info() -> &'static str {
    r#"{
    "nombre": "Hello Axum API",
    "version": "1.0.0",
    "autor": "Bootcamp Rust"
}"#
}

#[tokio::main]
async fn main() {
    // Crear el router con las rutas
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/greet/{name}", get(greet));

    // Crear el listener TCP
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("No se pudo iniciar el servidor en el puerto 3000");

    println!("🚀 Servidor iniciado en http://localhost:3000");
    println!();
    println!("📝 Endpoints disponibles:");
    println!("   GET /           - Página principal");
    println!("   GET /health     - Estado del servicio");
    println!("   GET /info       - Información del API");
    println!("   GET /greet/:n   - Personalized greeting");
    println!();
    println!("💡 Try: curl http://localhost:3000/greet/Rust");

    // Iniciar el servidor
    axum::serve(listener, app)
        .await
        .expect("Error al ejecutar el servidor");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_root() {
        let result = root().await;
        assert!(result.contains("Axum"));
    }

    #[tokio::test]
    async fn test_health() {
        let result = health().await;
        assert_eq!(result, "OK");
    }

    #[tokio::test]
    async fn test_greet() {
        let result = greet(Path("Ana".to_string())).await;
        assert!(result.contains("Ana"));
        assert!(result.contains("Hola"));
    }
}
