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
//! ## Documentación
//!
//! Swagger UI disponible en: `http://localhost:3000/swagger-ui`
//!
//! ## Filtros
//!
//! - `?completada=true` - Solo tareas completadas
//! - `?completada=false` - Solo tareas pendientes
//! - `?limite=10` - Limitar resultados
//! - `?offset=0` - Paginación

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use proyecto_api_tareas::{db, handlers, models, routes};

/// Documentación OpenAPI de la API de Tareas
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::listar,
        handlers::obtener,
        handlers::crear,
        handlers::actualizar,
        handlers::eliminar,
        handlers::estadisticas,
    ),
    components(
        schemas(
            models::Tarea,
            models::CrearTarea,
            models::ActualizarTarea,
            models::FiltroTareas,
            models::EstadisticasTareas,
            models::ErrorResponse,
        )
    ),
    tags(
        (name = "Tareas", description = "Endpoints de gestión de tareas"),
        (name = "Estadísticas", description = "Endpoints de estadísticas")
    ),
    info(
        title = "API de Tareas",
        version = "1.0.0",
        description = "API REST para gestión de tareas - Proyecto Final del Bootcamp Rust",
        contact(
            name = "Bootcamp Rust",
            url = "https://github.com/ergrato-dev/bc-rust"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    )
)]
struct ApiDoc;

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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("🌐 Servidor escuchando en http://localhost:3000");
    tracing::info!("");
    tracing::info!("📚 Swagger UI: http://localhost:3000/swagger-ui");
    tracing::info!("📄 OpenAPI JSON: http://localhost:3000/api-docs/openapi.json");
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
