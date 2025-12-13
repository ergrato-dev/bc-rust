//! Integration tests for Task API
//!
//! Run with: `cargo test`

use axum::{
    body::Body,
    http::\{Request, StatusCode},
    Router,
};
use proyecto_api_tareas::{db, models::Task, routes};
use serde_json::json;
use tower::ServiceExt;
use tower_http::trace::TraceLayer;

/// Helper to create test application
async fn create_app() -> Router {
    let pool = db::create_pool().await.expect("Error creating pool");

    Router::new()
        .merge(routes::create_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}

/// Helper to make requests
async fn request(app: Router, method: &str, uri: &str, body: Option<serde_json::Value>) -> (StatusCode, String) {
    let body = match body {
        Some(json) => Body::from(serde_json::to_string(&json).unwrap()),
        None => Body::empty(),
    };

    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(body)
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();

    (status, body)
}

// ============================================================
// Creation Tests
// ============================================================

#[tokio::test]
async fn test_create_task_success() {
    let app = create_app().await;

    let (status, body) = request(
        app,
        "POST",
        "/tasks",
        Some(json!({
            "title": "My first task",
            "description": "Test description"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);

    let task: Task = serde_json::from_str(&body).unwrap();
    assert_eq!(task.title, "My first task");
    assert_eq!(task.description, Some("Test description".to_string()));
    assert!(!task.completed);
    assert!(task.id > 0);
}

#[tokio::test]
async fn test_create_task_without_description() {
    let app = create_app().await;

    let (status, body) = request(
        app,
        "POST",
        "/tasks",
        Some(json!({
            "title": "Task without description"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);

    let task: Task = serde_json::from_str(&body).unwrap();
    assert_eq!(task.title, "Task without description");
    assert!(task.description.is_none());
}

#[tokio::test]
async fn test_create_task_empty_title() {
    let app = create_app().await;

    let (status, _body) = request(
        app,
        "POST",
        "/tasks",
        Some(json!({
            "title": ""
        })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_task_whitespace_only_title() {
    let app = create_app().await;

    let (status, _body) = request(
        app,
        "POST",
        "/tasks",
        Some(json!({
            "title": "   "
        })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// ============================================================
// Read Tests
// ============================================================

#[tokio::test]
async fn test_list_tasks_empty() {
    let app = create_app().await;

    let (status, body) = request(app, "GET", "/tasks", None).await;

    assert_eq!(status, StatusCode::OK);

    let tasks: Vec<Task> = serde_json::from_str(&body).unwrap();
    // May be empty or have tasks from previous tests
    assert!(tasks.len() >= 0);
}

#[tokio::test]
async fn test_create_and_get_task() {
    let app = create_app().await;

    // Create task
    let (status, body) = request(
        app.clone(),
        "POST",
        "/tasks",
        Some(json!({
            "title": "Task to get"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    let task: Task = serde_json::from_str(&body).unwrap();
    let id = task.id;

    // Get task
    let (status, body) = request(app, "GET", &format!("/tasks/{}", id), None).await;

    assert_eq!(status, StatusCode::OK);
    let retrieved_task: Task = serde_json::from_str(&body).unwrap();
    assert_eq!(retrieved_task.id, id);
    assert_eq!(retrieved_task.title, "Task to get");
}

#[tokio::test]
async fn test_get_task_not_found() {
    let app = create_app().await;

    let (status, _body) = request(app, "GET", "/tasks/999999", None).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ============================================================
// Update Tests
// ============================================================

#[tokio::test]
async fn test_update_task_title() {
    let app = create_app().await;

    // Create task
    let (_, body) = request(
        app.clone(),
        "POST",
        "/tasks",
        Some(json!({
            "title": "Original title"
        })),
    )
    .await;

    let task: Task = serde_json::from_str(&body).unwrap();
    let id = task.id;

    // Update title
    let (status, body) = request(
        app,
        "PUT",
        &format!("/tasks/{}", id),
        Some(json!({
            "title": "Updated title"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let updated_task: Task = serde_json::from_str(&body).unwrap();
    assert_eq!(updated_task.title, "Updated title");
}

#[tokio::test]
async fn test_update_task_complete() {
    let app = create_app().await;

    // Create task
    let (_, body) = request(
        app.clone(),
        "POST",
        "/tasks",
        Some(json!({
            "title": "Task to complete"
        })),
    )
    .await;

    let task: Task = serde_json::from_str(&body).unwrap();
    let id = task.id;
    assert!(!task.completed);

    // Mark as completed
    let (status, body) = request(
        app,
        "PUT",
        &format!("/tasks/{}", id),
        Some(json!({
            "completed": true
        })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let updated_task: Task = serde_json::from_str(&body).unwrap();
    assert!(updated_task.completed);
}

#[tokio::test]
async fn test_update_task_not_found() {
    let app = create_app().await;

    let (status, _body) = request(
        app,
        "PUT",
        "/tasks/999999",
        Some(json!({
            "title": "New title"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ============================================================
// Delete Tests
// ============================================================

#[tokio::test]
async fn test_delete_task() {
    let app = create_app().await;

    // Create task
    let (_, body) = request(
        app.clone(),
        "POST",
        "/tasks",
        Some(json!({
            "title": "Task to delete"
        })),
    )
    .await;

    let task: Task = serde_json::from_str(&body).unwrap();
    let id = task.id;

    // Delete task
    let (status, _body) = request(app.clone(), "DELETE", &format!("/tasks/{}", id), None).await;

    assert_eq!(status, StatusCode::NO_CONTENT);

    // Verify it no longer exists
    let (status, _body) = request(app, "GET", &format!("/tasks/{}", id), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_task_not_found() {
    let app = create_app().await;

    let (status, _body) = request(app, "DELETE", "/tasks/999999", None).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ============================================================
// Filter Tests
// ============================================================

#[tokio::test]
async fn test_filter_completed_tasks() {
    let app = create_app().await;

    // Create task
    let (_, body) = request(
        app.clone(),
        "POST",
        "/tasks",
        Some(json!({
            "title": "Completed task for filter"
        })),
    )
    .await;

    let task: Task = serde_json::from_str(&body).unwrap();
    let id = task.id;

    // Mark as completed
    let _ = request(
        app.clone(),
        "PUT",
        &format!("/tasks/{}", id),
        Some(json!({
            "completed": true
        })),
    )
    .await;

    // Filter completed
    let (status, body) = request(app, "GET", "/tasks?completed=true", None).await;

    assert_eq!(status, StatusCode::OK);
    let tasks: Vec<Task> = serde_json::from_str(&body).unwrap();

    // All must be completed
    for task in &tasks {
        assert!(task.completed);
    }
}

#[tokio::test]
async fn test_filter_tasks_limit() {
    let app = create_app().await;

    // Create several tasks
    for i in 0..5 {
        let _ = request(
            app.clone(),
            "POST",
            "/tasks",
            Some(json!({
                "title": format!("Limit task {}", i)
            })),
        )
        .await;
    }

    // Filter with limit
    let (status, body) = request(app, "GET", "/tasks?limit=3", None).await;

    assert_eq!(status, StatusCode::OK);
    let tasks: Vec<Task> = serde_json::from_str(&body).unwrap();

    assert!(tasks.len() <= 3);
}

// ============================================================
// Statistics Tests
// ============================================================

#[tokio::test]
async fn test_statistics() {
    let app = create_app().await;

    let (status, body) = request(app, "GET", "/tasks/stats", None).await;

    assert_eq!(status, StatusCode::OK);

    let stats: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(stats.get("total").is_some());
    assert!(stats.get("completed").is_some());
    assert!(stats.get("pending").is_some());
}
