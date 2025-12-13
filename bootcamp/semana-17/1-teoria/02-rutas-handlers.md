# 📖 Rutas y Handlers

## El Sistema de Rutas

El **Router** de Axum mapea URLs a handlers usando un sistema de rutas flexible:

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

let app = Router::new()
    .route("/", get(raiz))
    .route("/users", get(listar_usuarios).post(crear_usuario))
    .route("/users/:id", get(obtener_usuario)
                         .put(actualizar_usuario)
                         .delete(eliminar_usuario));
```

---

## 🎯 Diagrama Visual

![Rutas y Handlers](../0-assets/02-rutas-handlers.svg)

---

## Métodos HTTP

### Funciones de Routing

| Función | Método HTTP | Uso |
|---------|-------------|-----|
| `get()` | GET | Obtener recursos |
| `post()` | POST | Crear recursos |
| `put()` | PUT | Actualizar completo |
| `patch()` | PATCH | Actualizar parcial |
| `delete()` | DELETE | Eliminar |

### Encadenar Métodos

```rust
use axum::routing::{get, post, put, delete};

// Una ruta con múltiples métodos
Router::new()
    .route("/items", 
        get(listar)
            .post(crear)
    )
    .route("/items/:id",
        get(obtener)
            .put(actualizar)
            .delete(eliminar)
    );
```

---

## Parámetros de Ruta

### Sintaxis de Parámetros

```rust
// Parámetro simple: :nombre
"/users/:id"           // /users/123

// Múltiples parámetros
"/users/:user_id/posts/:post_id"  // /users/1/posts/42

// Wildcard (captura todo)
"/files/*path"         // /files/docs/rust/chapter1.md
```

### Extraer Parámetros

```rust
use axum::extract::Path;

// Un parámetro
async fn obtener_usuario(Path(id): Path<u64>) -> String {
    format!("Usuario {}", id)
}

// Múltiples parámetros como tupla
async fn obtener_post(
    Path((user_id, post_id)): Path<(u64, u64)>
) -> String {
    format!("Usuario {} - Post {}", user_id, post_id)
}

// Múltiples parámetros como struct
#[derive(Deserialize)]
struct PostParams {
    user_id: u64,
    post_id: u64,
}

async fn obtener_post_struct(
    Path(params): Path<PostParams>
) -> String {
    format!("Usuario {} - Post {}", params.user_id, params.post_id)
}
```

---

## Handlers en Detalle

### Anatomía de un Handler

```rust
// Handler = función async que retorna algo que implementa IntoResponse
async fn mi_handler(
    // Extractores (0 o más)
    extractor1: Extractor1,
    extractor2: Extractor2,
) -> impl IntoResponse {
    // Lógica
    // Retorno
}
```

### Ejemplos de Handlers

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};

// Handler simple
async fn health() -> &'static str {
    "OK"
}

// Handler con código de estado
async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

// Handler con JSON
#[derive(Serialize)]
struct Usuario {
    id: u64,
    nombre: String,
}

async fn obtener_usuario(Path(id): Path<u64>) -> Json<Usuario> {
    Json(Usuario {
        id,
        nombre: "Ana".to_string(),
    })
}

// Handler con tupla (status + body)
async fn crear_usuario() -> (StatusCode, Json<Usuario>) {
    let usuario = Usuario {
        id: 1,
        nombre: "Nuevo".to_string(),
    };
    (StatusCode::CREATED, Json(usuario))
}
```

---

## Tipos de Respuesta

### Respuestas Comunes

```rust
use axum::{
    http::StatusCode,
    response::{Html, Json, Redirect, IntoResponse},
};

// Texto plano
async fn texto() -> &'static str {
    "Hola mundo"
}

// HTML
async fn html() -> Html<&'static str> {
    Html("<h1>Hola</h1>")
}

// JSON
async fn json() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "mensaje": "Hola"
    }))
}

// Redirección
async fn redirigir() -> Redirect {
    Redirect::to("/nueva-ruta")
}

// Headers personalizados
use axum::http::header;

async fn con_headers() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/plain")],
        "Contenido con headers"
    )
}
```

---

## Manejo de Errores

### Result en Handlers

```rust
use axum::{
    http::StatusCode,
    response::{Json, IntoResponse},
};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Handler que puede fallar
async fn obtener_usuario(
    Path(id): Path<u64>
) -> Result<Json<Usuario>, (StatusCode, Json<ErrorResponse>)> {
    if id == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "ID inválido".to_string()
            })
        ));
    }
    
    // Simular búsqueda
    if id > 100 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Usuario {} no encontrado", id)
            })
        ));
    }
    
    Ok(Json(Usuario {
        id,
        nombre: "Encontrado".to_string()
    }))
}
```

### Tipo de Error Personalizado

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Error personalizado
enum ApiError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

// Usar en handlers
async fn obtener_usuario(Path(id): Path<u64>) -> Result<Json<Usuario>, ApiError> {
    if id == 0 {
        return Err(ApiError::BadRequest("ID no puede ser 0".into()));
    }
    
    buscar_usuario(id)
        .ok_or_else(|| ApiError::NotFound(format!("Usuario {} no encontrado", id)))
        .map(Json)
}
```

---

## Organización de Rutas

### Rutas Anidadas

```rust
// Módulo de usuarios
fn usuarios_routes() -> Router {
    Router::new()
        .route("/", get(listar_usuarios).post(crear_usuario))
        .route("/:id", get(obtener_usuario)
                       .put(actualizar_usuario)
                       .delete(eliminar_usuario))
}

// Módulo de posts
fn posts_routes() -> Router {
    Router::new()
        .route("/", get(listar_posts).post(crear_post))
        .route("/:id", get(obtener_post).delete(eliminar_post))
}

// Router principal
fn crear_app() -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/users", usuarios_routes())
        .nest("/posts", posts_routes())
}
```

### Resultado

```
GET  /health           -> health()
GET  /users            -> listar_usuarios()
POST /users            -> crear_usuario()
GET  /users/:id        -> obtener_usuario()
PUT  /users/:id        -> actualizar_usuario()
DELETE /users/:id      -> eliminar_usuario()
GET  /posts            -> listar_posts()
POST /posts            -> crear_post()
GET  /posts/:id        -> obtener_post()
DELETE /posts/:id      -> eliminar_post()
```

---

## Fallback Handler

```rust
// Handler para rutas no encontradas
async fn handler_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Ruta no encontrada")
}

let app = Router::new()
    .route("/", get(raiz))
    .route("/users", get(usuarios))
    .fallback(handler_404);
```

---

## Ejemplo Completo: API CRUD

```rust
use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Producto {
    id: u64,
    nombre: String,
    precio: f64,
}

// Handlers
async fn listar() -> Json<Vec<Producto>> {
    Json(vec![
        Producto { id: 1, nombre: "Laptop".into(), precio: 999.99 },
        Producto { id: 2, nombre: "Mouse".into(), precio: 29.99 },
    ])
}

async fn obtener(Path(id): Path<u64>) -> Result<Json<Producto>, StatusCode> {
    if id == 1 {
        Ok(Json(Producto { id: 1, nombre: "Laptop".into(), precio: 999.99 }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn crear(Json(producto): Json<Producto>) -> (StatusCode, Json<Producto>) {
    (StatusCode::CREATED, Json(producto))
}

async fn actualizar(
    Path(id): Path<u64>,
    Json(mut producto): Json<Producto>,
) -> Json<Producto> {
    producto.id = id;
    Json(producto)
}

async fn eliminar(Path(id): Path<u64>) -> StatusCode {
    println!("Eliminando producto {}", id);
    StatusCode::NO_CONTENT
}

// Router
fn crear_app() -> Router {
    Router::new()
        .route("/productos", get(listar).post(crear))
        .route("/productos/:id", get(obtener)
                                 .put(actualizar)
                                 .delete(eliminar))
}
```

---

## Resumen

| Concepto | Descripción |
|----------|-------------|
| **Router** | Mapea URLs a handlers |
| **route()** | Define una ruta con métodos |
| **nest()** | Agrupa rutas con prefijo |
| **Path** | Extrae parámetros de URL |
| **fallback()** | Handler para rutas no encontradas |
| **IntoResponse** | Trait para respuestas |

### Próximo tema

En el siguiente archivo aprenderás sobre **Extractores** para obtener datos del request.
