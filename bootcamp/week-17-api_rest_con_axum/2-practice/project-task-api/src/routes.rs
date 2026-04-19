//! API Routes Definition

use axum::{routing::get, Router};
use sqlx::SqlitePool;

use crate::handlers;

/// Create API router
pub fn create_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/tasks", get(handlers::list_tasks).post(handlers::create_task))
        .route("/tasks/stats", get(handlers::get_stats))
        .route(
            "/tasks/{id}",
            get(handlers::get_task)
                .put(handlers::update_task)
                .delete(handlers::delete_task),
        )
}
