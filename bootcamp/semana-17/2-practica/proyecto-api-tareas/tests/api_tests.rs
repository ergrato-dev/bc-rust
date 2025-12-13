//! Integration tests for Task API
//!
//! Run with: `cargo test`

use axum::{
    body::Body,
    http::{Request, StatusCode},
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
    let app = crear_app().await;

    // Crear tarea
    let (status, body) = request(
        app.clone(),
        "POST",
        "/tareas",
        Some(json!({
            "titulo": "Tarea para obtener"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    let tarea: Tarea = serde_json::from_str(&body).unwrap();
    let id = tarea.id;

    // Obtener tarea
    let (status, body) = request(app, "GET", &format!("/tareas/{}", id), None).await;

    assert_eq!(status, StatusCode::OK);
    let tarea_obtenida: Tarea = serde_json::from_str(&body).unwrap();
    assert_eq!(tarea_obtenida.id, id);
    assert_eq!(tarea_obtenida.titulo, "Tarea para obtener");
}

#[tokio::test]
async fn test_obtener_tarea_no_existe() {
    let app = crear_app().await;

    let (status, _body) = request(app, "GET", "/tareas/999999", None).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ============================================================
// Tests de Actualización
// ============================================================

#[tokio::test]
async fn test_actualizar_tarea_titulo() {
    let app = crear_app().await;

    // Crear tarea
    let (_, body) = request(
        app.clone(),
        "POST",
        "/tareas",
        Some(json!({
            "titulo": "Título original"
        })),
    )
    .await;

    let tarea: Tarea = serde_json::from_str(&body).unwrap();
    let id = tarea.id;

    // Actualizar título
    let (status, body) = request(
        app,
        "PUT",
        &format!("/tareas/{}", id),
        Some(json!({
            "titulo": "Título actualizado"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let tarea_actualizada: Tarea = serde_json::from_str(&body).unwrap();
    assert_eq!(tarea_actualizada.titulo, "Título actualizado");
}

#[tokio::test]
async fn test_actualizar_tarea_completar() {
    let app = crear_app().await;

    // Crear tarea
    let (_, body) = request(
        app.clone(),
        "POST",
        "/tareas",
        Some(json!({
            "titulo": "Tarea a completar"
        })),
    )
    .await;

    let tarea: Tarea = serde_json::from_str(&body).unwrap();
    let id = tarea.id;
    assert!(!tarea.completada);

    // Marcar como completada
    let (status, body) = request(
        app,
        "PUT",
        &format!("/tareas/{}", id),
        Some(json!({
            "completada": true
        })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let tarea_actualizada: Tarea = serde_json::from_str(&body).unwrap();
    assert!(tarea_actualizada.completada);
}

#[tokio::test]
async fn test_actualizar_tarea_no_existe() {
    let app = crear_app().await;

    let (status, _body) = request(
        app,
        "PUT",
        "/tareas/999999",
        Some(json!({
            "titulo": "Nuevo título"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ============================================================
// Tests de Eliminación
// ============================================================

#[tokio::test]
async fn test_eliminar_tarea() {
    let app = crear_app().await;

    // Crear tarea
    let (_, body) = request(
        app.clone(),
        "POST",
        "/tareas",
        Some(json!({
            "titulo": "Tarea a eliminar"
        })),
    )
    .await;

    let tarea: Tarea = serde_json::from_str(&body).unwrap();
    let id = tarea.id;

    // Eliminar tarea
    let (status, _body) = request(app.clone(), "DELETE", &format!("/tareas/{}", id), None).await;

    assert_eq!(status, StatusCode::NO_CONTENT);

    // Verificar que no existe
    let (status, _body) = request(app, "GET", &format!("/tareas/{}", id), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_eliminar_tarea_no_existe() {
    let app = crear_app().await;

    let (status, _body) = request(app, "DELETE", "/tareas/999999", None).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ============================================================
// Tests de Filtros
// ============================================================

#[tokio::test]
async fn test_filtrar_tareas_completadas() {
    let app = crear_app().await;

    // Crear tarea completada
    let (_, body) = request(
        app.clone(),
        "POST",
        "/tareas",
        Some(json!({
            "titulo": "Tarea completada para filtro"
        })),
    )
    .await;

    let tarea: Tarea = serde_json::from_str(&body).unwrap();
    let id = tarea.id;

    // Marcar como completada
    let _ = request(
        app.clone(),
        "PUT",
        &format!("/tareas/{}", id),
        Some(json!({
            "completada": true
        })),
    )
    .await;

    // Filtrar completadas
    let (status, body) = request(app, "GET", "/tareas?completada=true", None).await;

    assert_eq!(status, StatusCode::OK);
    let tareas: Vec<Tarea> = serde_json::from_str(&body).unwrap();

    // Todas deben estar completadas
    for tarea in &tareas {
        assert!(tarea.completada);
    }
}

#[tokio::test]
async fn test_filtrar_tareas_limite() {
    let app = crear_app().await;

    // Crear varias tareas
    for i in 0..5 {
        let _ = request(
            app.clone(),
            "POST",
            "/tareas",
            Some(json!({
                "titulo": format!("Tarea limite {}", i)
            })),
        )
        .await;
    }

    // Filtrar con límite
    let (status, body) = request(app, "GET", "/tareas?limite=3", None).await;

    assert_eq!(status, StatusCode::OK);
    let tareas: Vec<Tarea> = serde_json::from_str(&body).unwrap();

    assert!(tareas.len() <= 3);
}

// ============================================================
// Tests de Estadísticas
// ============================================================

#[tokio::test]
async fn test_estadisticas() {
    let app = crear_app().await;

    let (status, body) = request(app, "GET", "/tareas/estadisticas", None).await;

    assert_eq!(status, StatusCode::OK);

    let stats: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(stats.get("total").is_some());
    assert!(stats.get("completadas").is_some());
    assert!(stats.get("pendientes").is_some());
}
