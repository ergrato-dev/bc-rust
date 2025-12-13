//! Práctica 02: CRUD de Usuarios
//!
//! API REST completa con operaciones CRUD usando estado en memoria.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::RwLock;

// =============================================================================
// MODELS
// =============================================================================

/// System user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub active: bool,
}

/// DTO to create user (without ID)
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

/// DTO to update user (optional fields)
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub active: Option<bool>,
}

// =============================================================================
// STATE
// =============================================================================

/// Application shared state
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<u64, User>>>,
    pub counter: Arc<RwLock<u64>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            counter: Arc::new(RwLock::new(0)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// HANDLERS
// =============================================================================

/// GET /users - List all users
async fn list_users(
    State(state): State<AppState>,
) -> Json<Vec<User>> {
    let users = state.users.read().await;
    let list: Vec<User> = users.values().cloned().collect();
    Json(list)
}

/// GET /users/:id - Get a user by ID
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.read().await;
    
    users
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// POST /users - Create new user
async fn create_user(
    State(state): State<AppState>,
    Json(data): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // Generate new ID
    let mut counter = state.counter.write().await;
    *counter += 1;
    let id = *counter;
    
    // Create user
    let user = User {
        id,
        name: data.name,
        email: data.email,
        active: true,
    };
    
    // Save in state
    let mut users = state.users.write().await;
    users.insert(id, user.clone());
    
    (StatusCode::CREATED, Json(user))
}

/// PUT /users/:id - Update existing user
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(data): Json<UpdateUser>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.write().await;
    
    let user = users
        .get_mut(&id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Update provided fields
    if let Some(name) = data.name {
        user.name = name;
    }
    if let Some(email) = data.email {
        user.email = email;
    }
    if let Some(active) = data.active {
        user.active = active;
    }
    
    Ok(Json(user.clone()))
}

/// DELETE /users/:id - Delete user
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut users = state.users.write().await;
    
    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// =============================================================================
// ROUTER
// =============================================================================

/// Create application router
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", 
            get(get_user)
                .put(update_user)
                .delete(delete_user)
        )
        .with_state(state)
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not start server");

    println!("🚀 User CRUD API");
    println!("   http://localhost:3000");
    println!();
    println!("📝 Endpoints:");
    println!("   GET    /users      - List all");
    println!("   POST   /users      - Create user");
    println!("   GET    /users/:id  - Get one");
    println!("   PUT    /users/:id  - Update");
    println!("   DELETE /users/:id  - Delete");

    axum::serve(listener, app).await.unwrap();
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    fn create_test_app() -> Router {
        create_app(AppState::new())
    }

    #[tokio::test]
    async fn test_list_empty() {
        let app = create_test_app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_user() {
        let app = create_test_app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"name":"Ana","email":"ana@test.com"}"#))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let app = create_test_app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/999")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
