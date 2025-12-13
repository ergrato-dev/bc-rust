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
// MODELOS
// =============================================================================

/// Usuario del sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub id: u64,
    pub nombre: String,
    pub email: String,
    pub activo: bool,
}

/// DTO para crear usuario (sin ID)
#[derive(Debug, Deserialize)]
pub struct CrearUsuario {
    pub nombre: String,
    pub email: String,
}

/// DTO para actualizar usuario (campos opcionales)
#[derive(Debug, Deserialize)]
pub struct ActualizarUsuario {
    pub nombre: Option<String>,
    pub email: Option<String>,
    pub activo: Option<bool>,
}

// =============================================================================
// ESTADO
// =============================================================================

/// Estado compartido de la aplicación
#[derive(Clone)]
pub struct AppState {
    pub usuarios: Arc<RwLock<HashMap<u64, Usuario>>>,
    pub contador: Arc<RwLock<u64>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            usuarios: Arc::new(RwLock::new(HashMap::new())),
            contador: Arc::new(RwLock::new(0)),
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

/// GET /usuarios - Listar todos los usuarios
async fn listar_usuarios(
    State(state): State<AppState>,
) -> Json<Vec<Usuario>> {
    let usuarios = state.usuarios.read().await;
    let lista: Vec<Usuario> = usuarios.values().cloned().collect();
    Json(lista)
}

/// GET /usuarios/:id - Obtener un usuario por ID
async fn obtener_usuario(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Usuario>, StatusCode> {
    let usuarios = state.usuarios.read().await;
    
    usuarios
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// POST /usuarios - Crear nuevo usuario
async fn crear_usuario(
    State(state): State<AppState>,
    Json(datos): Json<CrearUsuario>,
) -> (StatusCode, Json<Usuario>) {
    // Generar nuevo ID
    let mut contador = state.contador.write().await;
    *contador += 1;
    let id = *contador;
    
    // Crear usuario
    let usuario = Usuario {
        id,
        nombre: datos.nombre,
        email: datos.email,
        activo: true,
    };
    
    // Guardar en el estado
    let mut usuarios = state.usuarios.write().await;
    usuarios.insert(id, usuario.clone());
    
    (StatusCode::CREATED, Json(usuario))
}

/// PUT /usuarios/:id - Actualizar usuario existente
async fn actualizar_usuario(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(datos): Json<ActualizarUsuario>,
) -> Result<Json<Usuario>, StatusCode> {
    let mut usuarios = state.usuarios.write().await;
    
    let usuario = usuarios
        .get_mut(&id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Actualizar campos proporcionados
    if let Some(nombre) = datos.nombre {
        usuario.nombre = nombre;
    }
    if let Some(email) = datos.email {
        usuario.email = email;
    }
    if let Some(activo) = datos.activo {
        usuario.activo = activo;
    }
    
    Ok(Json(usuario.clone()))
}

/// DELETE /usuarios/:id - Eliminar usuario
async fn eliminar_usuario(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut usuarios = state.usuarios.write().await;
    
    if usuarios.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// =============================================================================
// ROUTER
// =============================================================================

/// Crear el router de la aplicación
pub fn crear_app(state: AppState) -> Router {
    Router::new()
        .route("/usuarios", get(listar_usuarios).post(crear_usuario))
        .route("/usuarios/{id}", 
            get(obtener_usuario)
                .put(actualizar_usuario)
                .delete(eliminar_usuario)
        )
        .with_state(state)
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = crear_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("No se pudo iniciar el servidor");

    println!("🚀 API CRUD de Usuarios");
    println!("   http://localhost:3000");
    println!();
    println!("📝 Endpoints:");
    println!("   GET    /usuarios      - Listar todos");
    println!("   POST   /usuarios      - Crear usuario");
    println!("   GET    /usuarios/:id  - Obtener uno");
    println!("   PUT    /usuarios/:id  - Actualizar");
    println!("   DELETE /usuarios/:id  - Eliminar");

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

    fn crear_app_test() -> Router {
        crear_app(AppState::new())
    }

    #[tokio::test]
    async fn test_listar_vacio() {
        let app = crear_app_test();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/usuarios")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_crear_usuario() {
        let app = crear_app_test();
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/usuarios")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"nombre":"Ana","email":"ana@test.com"}"#))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_usuario_no_encontrado() {
        let app = crear_app_test();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/usuarios/999")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
