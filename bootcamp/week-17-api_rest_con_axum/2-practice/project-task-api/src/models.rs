//! Data models

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// Task stored in the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Task {
    /// Unique task ID
    #[schema(example = 1)]
    pub id: i64,
    /// Task title
    #[schema(example = "Learn Rust")]
    pub title: String,
    /// Optional description
    #[schema(example = "Complete the 17-week bootcamp")]
    pub description: Option<String>,
    /// Completion status
    #[schema(example = false)]
    pub completed: bool,
    /// Creation timestamp
    #[schema(example = "2025-01-15 10:30:00")]
    pub created_at: String,
    /// Last update timestamp
    #[schema(example = "2025-01-15 12:00:00")]
    pub updated_at: String,
}

/// DTO for creating a task
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTask {
    /// Task title (required)
    #[schema(example = "Learn Rust")]
    pub title: String,
    /// Optional task description
    #[schema(example = "Complete the bootcamp")]
    pub description: Option<String>,
}

/// DTO for updating a task
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTask {
    /// New title (optional)
    #[schema(example = "Master Rust")]
    pub title: Option<String>,
    /// New description (optional)
    #[schema(example = "Now I'm an expert")]
    pub description: Option<String>,
    /// New completion status (optional)
    #[schema(example = true)]
    pub completed: Option<bool>,
}

/// Query filters
#[derive(Debug, Deserialize, Default, ToSchema)]
pub struct TaskFilters {
    /// Filter by completion status
    #[schema(example = false)]
    pub completed: Option<bool>,
    /// Results limit
    #[schema(example = 10)]
    pub limit: Option<i64>,
    /// Pagination offset
    #[schema(example = 0)]
    pub offset: Option<i64>,
}

/// Task statistics
#[derive(Debug, Serialize, ToSchema)]
pub struct TaskStats {
    /// Total tasks
    #[schema(example = 100)]
    pub total: i64,
    /// Completed tasks
    #[schema(example = 75)]
    pub completed: i64,
    /// Pending tasks
    #[schema(example = 25)]
    pub pending: i64,
}

/// API error response
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    #[schema(example = "Task not found")]
    pub error: String,
    /// HTTP status code
    #[schema(example = 404)]
    pub code: u16,
}
