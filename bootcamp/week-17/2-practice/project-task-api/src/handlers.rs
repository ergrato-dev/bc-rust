//! Task API Handlers

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use sqlx::SqlitePool;

use crate::error::{ApiError, Result};
use crate::models::{CreateTask, Task, TaskFilters, TaskStats, UpdateTask};

/// List all tasks
///
/// Gets a list of tasks with support for filters and pagination.
#[utoipa::path(
    get,
    path = "/tasks",
    params(
        ("completed" = Option<bool>, Query, description = "Filter by completion status"),
        ("limit" = Option<i64>, Query, description = "Result limit (default: 100)"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination (default: 0)")
    ),
    responses(
        (status = 200, description = "List of tasks", body = Vec<Task>),
        (status = 500, description = "Internal error", body = crate::models::ErrorResponse)
    ),
    tag = "Tasks"
)]
pub async fn list_tasks(
    State(pool): State<SqlitePool>,
    Query(filters): Query<TaskFilters>,
) -> Result<Json<Vec<Task>>> {
    let limit = filters.limit.unwrap_or(100);
    let offset = filters.offset.unwrap_or(0);

    let tasks = match filters.completed {
        Some(completed) => {
            sqlx::query_as::<_, Task>(
                "SELECT id, title, description, completed, created_at, updated_at FROM tasks WHERE completed = ? ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(completed)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, Task>(
                "SELECT id, title, description, completed, created_at, updated_at FROM tasks ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?
        }
    };

    Ok(Json(tasks))
}

/// Get a task by ID
///
/// Returns details of a specific task.
#[utoipa::path(
    get,
    path = "/tasks/{id}",
    params(
        ("id" = i64, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task found", body = Task),
        (status = 404, description = "Task not found", body = crate::models::ErrorResponse)
    ),
    tag = "Tasks"
)]
pub async fn get_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Task>> {
    let task = sqlx::query_as::<_, Task>("SELECT id, title, description, completed, created_at, updated_at FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Task {} not found", id)))?;

    Ok(Json(task))
}

/// Create a new task
///
/// Creates a task with the provided title.
#[utoipa::path(
    post,
    path = "/tasks",
    request_body = CreateTask,
    responses(
        (status = 201, description = "Task created successfully", body = Task),
        (status = 400, description = "Validation error", body = crate::models::ErrorResponse)
    ),
    tag = "Tasks"
)]
pub async fn create_task(
    State(pool): State<SqlitePool>,
    Json(data): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>)> {
    // Validation
    if data.title.trim().is_empty() {
        return Err(ApiError::Validation("Title is required".into()));
    }

    if data.title.len() > 200 {
        return Err(ApiError::Validation(
            "Title cannot exceed 200 characters".into(),
        ));
    }

    let result = sqlx::query("INSERT INTO tasks (title, description) VALUES (?, ?)")
        .bind(&data.title)
        .bind(&data.description)
        .execute(&pool)
        .await?;

    let id = result.last_insert_rowid();

    let task = sqlx::query_as::<_, Task>("SELECT id, title, description, completed, created_at, updated_at FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(task)))
}

/// Update an existing task
///
/// Updates one or more fields of a task.
#[utoipa::path(
    put,
    path = "/tasks/{id}",
    params(
        ("id" = i64, Path, description = "ID of the task to update")
    ),
    request_body = UpdateTask,
    responses(
        (status = 200, description = "Task updated", body = Task),
        (status = 400, description = "Validation error", body = crate::models::ErrorResponse),
        (status = 404, description = "Task not found", body = crate::models::ErrorResponse)
    ),
    tag = "Tasks"
)]
pub async fn update_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(data): Json<UpdateTask>,
) -> Result<Json<Task>> {
    // Verify it exists
    let exists = sqlx::query("SELECT id FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;;

    if exists.is_none() {
        return Err(ApiError::NotFound(format!("Task {} not found", id)));
    }

    // Update provided fields
    if let Some(title) = &data.title {
        if title.trim().is_empty() {
            return Err(ApiError::Validation("Title cannot be empty".into()));
        }
        sqlx::query(
            "UPDATE tasks SET title = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(title)
        .bind(id)
        .execute(&pool)
        .await?;
    }

    if let Some(description) = &data.description {
        sqlx::query(
            "UPDATE tasks SET description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(description)
        .bind(id)
        .execute(&pool)
        .await?;
    }

    if let Some(completed) = data.completed {
        sqlx::query(
            "UPDATE tasks SET completed = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(completed)
        .bind(id)
        .execute(&pool)
        .await?;
    }

    // Get updated task
    let task = sqlx::query_as::<_, Task>("SELECT id, title, description, completed, created_at, updated_at FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(task))
}

/// Delete a task
///
/// Permanently removes a task from the database.
#[utoipa::path(
    delete,
    path = "/tasks/{id}",
    params(
        ("id" = i64, Path, description = "ID of the task to delete")
    ),
    responses(
        (status = 204, description = "Task deleted successfully"),
        (status = 404, description = "Task not found", body = crate::models::ErrorResponse)
    ),
    tag = "Tasks"
)]
pub async fn delete_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("Task {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Get task statistics
///
/// Returns total count, completed and pending tasks.
#[utoipa::path(
    get,
    path = "/tasks/stats",
    responses(
        (status = 200, description = "Task statistics", body = TaskStats)
    ),
    tag = "Statistics"
)]
pub async fn get_stats(State(pool): State<SqlitePool>) -> Result<Json<TaskStats>> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
        .fetch_one(&pool)
        .await?;

    let completed_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE completed = TRUE")
            .fetch_one(&pool)
            .await?;

    Ok(Json(TaskStats {
        total: total.0,
        completed: completed_count.0,
        pending: total.0 - completed_count.0,
    }))
}
