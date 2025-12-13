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

/// Handler para la ruta raíz
async fn raiz() -> &'static str {
    "¡Bienvenido a mi primera API con Axum! 🦀"
}

/// Handler de salud del servicio
async fn health() -> &'static str {
    "OK"
}

/// Handler con parámetro de ruta
///
/// # Ejemplo
/// GET /saludo/Ana -> "¡Hola, Ana! 👋"
async fn saludar(Path(nombre): Path<String>) -> String {
    format!("¡Hola, {}! 👋", nombre)
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
        .route("/", get(raiz))
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/saludo/{nombre}", get(saludar));

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
    println!("   GET /saludo/:n  - Saludo personalizado");
    println!();
    println!("💡 Prueba: curl http://localhost:3000/saludo/Rust");

    // Iniciar el servidor
    axum::serve(listener, app)
        .await
        .expect("Error al ejecutar el servidor");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_raiz() {
        let resultado = raiz().await;
        assert!(resultado.contains("Axum"));
    }

    #[tokio::test]
    async fn test_health() {
        let resultado = health().await;
        assert_eq!(resultado, "OK");
    }

    #[tokio::test]
    async fn test_saludar() {
        let resultado = saludar(Path("Ana".to_string())).await;
        assert!(resultado.contains("Ana"));
        assert!(resultado.contains("Hola"));
    }
}
