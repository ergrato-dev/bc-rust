//! Task API - Rust Bootcamp Final Project
//!
//! Complete REST API with SQLite for task management.
//!
//! ## Endpoints
//!
//! | Method | Route | Description |
//! |--------|------|-------------|
//! | GET | /tasks | List all tasks |
//! | POST | /tasks | Create new task |
//! | GET | /tasks/:id | Get task by ID |
//! | PUT | /tasks/:id | Update task |
//! | DELETE | /tasks/:id | Delete task |
//! | GET | /tasks/stats | Statistics |
//!
//! ## Documentation
//!
//! Swagger UI available at: `http://localhost:3000/swagger-ui`
//!
//! ## Filters
//!
//! - `?completed=true` - Only completed tasks
//! - `?completed=false` - Only pending tasks
//! - `?limit=10` - Limit results
//! - `?offset=0` - Pagination

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use project_task_api::{db, handlers, models, routes};

/// Task API OpenAPI Documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::list_tasks,
        handlers::get_task,
        handlers::create_task,
        handlers::update_task,
        handlers::delete_task,
        handlers::get_stats,
    ),
    components(
        schemas(
            models::Task,
            models::CreateTask,
            models::UpdateTask,
            models::TaskFilters,
            models::TaskStats,
            models::ErrorResponse,
        )
    ),
    tags(
        (name = "Tasks", description = "Task management endpoints"),
        (name = "Statistics", description = "Statistics endpoints")
    ),
    info(
        title = "Task API",
        version = "1.0.0",
        description = "REST API for task management - Rust Bootcamp Final Project",
        contact(
            name = "Rust Bootcamp",
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
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("🚀 Starting Task API...");

    // Create SQLite connection pool
    let pool = db::create_pool().await?;
    tracing::info!("✅ SQLite connection established");

    // Build application
    let app = Router::new()
        .merge(routes::create_routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("🌐 Server listening on http://localhost:3000");
    tracing::info!("");
    tracing::info!("📚 Swagger UI: http://localhost:3000/swagger-ui");
    tracing::info!("📄 OpenAPI JSON: http://localhost:3000/api-docs/openapi.json");
    tracing::info!("");
    tracing::info!("📝 Available endpoints:");
    tracing::info!("   GET    /tasks         - List tasks");
    tracing::info!("   POST   /tasks         - Create task");
    tracing::info!("   GET    /tasks/:id     - Get task");
    tracing::info!("   PUT    /tasks/:id     - Update task");
    tracing::info!("   DELETE /tasks/:id     - Delete task");
    tracing::info!("   GET    /tasks/stats   - Statistics");
    tracing::info!("");
    tracing::info!("🔍 Filters: ?completed=true|false&limit=N&offset=N");
    tracing::info!("");
    tracing::info!("💡 Try:");
    tracing::info!(r#"   curl -X POST localhost:3000/tasks -H "Content-Type: application/json" -d '{{"title":"My task"}}'"#);

    axum::serve(listener, app).await?;

    Ok(())
}
