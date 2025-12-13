//! API de Tareas - Proyecto Final del Bootcamp Rust
//!
//! API REST completa con SQLite para gestión de tareas.
//!
//! ## Endpoints
//!
//! | Método | Ruta | Descripción |
//! |--------|------|-------------|
//! | GET | /tareas | Listar todas las tareas |
//! | POST | /tareas | Crear nueva tarea |
//! | GET | /tareas/:id | Obtener tarea por ID |
//! | PUT | /tareas/:id | Actualizar tarea |
//! | DELETE | /tareas/:id | Eliminar tarea |
//! | GET | /tareas/estadisticas | Estadísticas |
//!
//! ## Filtros
//!
//! - `?completada=true` - Solo tareas completadas
//! - `?completada=false` - Solo tareas pendientes
//! - `?limite=10` - Limitar resultados
//! - `?offset=0` - Paginación

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use proyecto_api_tareas::{db, routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("🚀 Iniciando API de Tareas...");

    // Crear pool de conexiones SQLite
    let pool = db::crear_pool().await?;
    tracing::info!("✅ Conexión a SQLite establecida");

    // Construir aplicación
    let app = Router::new()
        .merge(routes::crear_rutas())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("🌐 Servidor escuchando en http://localhost:3000");
    tracing::info!("");
    tracing::info!("📝 Endpoints disponibles:");
    tracing::info!("   GET    /tareas              - Listar tareas");
    tracing::info!("   POST   /tareas              - Crear tarea");
    tracing::info!("   GET    /tareas/:id          - Obtener tarea");
    tracing::info!("   PUT    /tareas/:id          - Actualizar tarea");
    tracing::info!("   DELETE /tareas/:id          - Eliminar tarea");
    tracing::info!("   GET    /tareas/estadisticas - Estadísticas");
    tracing::info!("");
    tracing::info!("🔍 Filtros: ?completada=true|false&limite=N&offset=N");
    tracing::info!("");
    tracing::info!("💡 Prueba:");
    tracing::info!(r#"   curl -X POST localhost:3000/tareas -H "Content-Type: application/json" -d '{{"titulo":"Mi tarea"}}'"#);

    axum::serve(listener, app).await?;

    Ok(())
}
