# 📖 Estado y Capas (State & Layers)

## Estado Compartido

En una API REST, necesitamos compartir datos entre handlers:
- Conexiones a base de datos
- Configuración
- Cachés
- Contadores

Axum usa **`State<T>`** para esto, donde `T` debe implementar `Clone`.

---

## 🎯 Diagrama Visual

![Estado y Capas](../0-assets/04-estado-capas.svg)

---

## Definir el Estado

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

// Estado de la aplicación
#[derive(Clone)]
struct AppState {
    // Datos en memoria (thread-safe)
    usuarios: Arc<RwLock<HashMap<u64, Usuario>>>,
    // Configuración (inmutable después de inicio)
    config: Arc<Config>,
}

#[derive(Clone)]
struct Config {
    nombre_app: String,
    version: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Usuario {
    id: u64,
    nombre: String,
    email: String,
}
```

---

## Crear e Inyectar Estado

```rust
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // Crear estado inicial
    let state = AppState {
        usuarios: Arc::new(RwLock::new(HashMap::new())),
        config: Arc::new(Config {
            nombre_app: "Mi API".to_string(),
            version: "1.0.0".to_string(),
        }),
    };

    // Inyectar en el router
    let app = Router::new()
        .route("/", get(info))
        .route("/users", get(listar).post(crear))
        .route("/users/:id", get(obtener))
        .with_state(state);  // <-- Inyectar estado

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
```

---

## Usar Estado en Handlers

```rust
use axum::extract::{Path, State};

// Handler que lee el estado
async fn info(State(state): State<AppState>) -> String {
    format!(
        "{} v{}",
        state.config.nombre_app,
        state.config.version
    )
}

// Handler que lee datos
async fn listar(State(state): State<AppState>) -> Json<Vec<Usuario>> {
    let usuarios = state.usuarios.read().await;
    Json(usuarios.values().cloned().collect())
}

// Handler que lee un item
async fn obtener(
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

// Handler que modifica datos
async fn crear(
    State(state): State<AppState>,
    Json(mut usuario): Json<Usuario>,
) -> (StatusCode, Json<Usuario>) {
    let mut usuarios = state.usuarios.write().await;
    let id = usuarios.len() as u64 + 1;
    usuario.id = id;
    usuarios.insert(id, usuario.clone());
    (StatusCode::CREATED, Json(usuario))
}
```

---

## Capas (Layers) y Middleware

Las **capas** envuelven handlers para añadir funcionalidad transversal:

```
┌────────────────────────────────────────────────────────────┐
│                      REQUEST                               │
│                         │                                  │
│    ┌────────────────────▼────────────────────┐            │
│    │           Layer: Logging                │            │
│    │    ┌───────────────▼───────────────┐    │            │
│    │    │      Layer: Auth              │    │            │
│    │    │    ┌──────────▼──────────┐    │    │            │
│    │    │    │      Handler        │    │    │            │
│    │    │    └──────────┬──────────┘    │    │            │
│    │    └───────────────┼───────────────┘    │            │
│    └────────────────────┼────────────────────┘            │
│                         ▼                                  │
│                     RESPONSE                               │
└────────────────────────────────────────────────────────────┘
```

---

## Middleware de Logging

```rust
use tower_http::trace::TraceLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Inicializar logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(raiz))
        .route("/users", get(listar))
        // Añadir layer de tracing
        .layer(TraceLayer::new_for_http());

    // Los requests ahora se loguean automáticamente
}
```

### Configuración Detallada de Trace

```rust
use tower_http::trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse};
use tracing::Level;

let app = Router::new()
    .route("/", get(raiz))
    .layer(
        TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
    );
```

---

## Middleware de CORS

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

let app = Router::new()
    .route("/api/data", get(data))
    .layer(cors);
```

### CORS Restringido

```rust
use tower_http::cors::CorsLayer;
use axum::http::{Method, header};

let cors = CorsLayer::new()
    .allow_origin(["https://mi-frontend.com".parse().unwrap()])
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([header::CONTENT_TYPE]);
```

---

## Middleware Personalizado

### Función Middleware

```rust
use axum::{
    middleware::{self, Next},
    http::Request,
    response::Response,
};

async fn logging_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    println!("➡️  {} {}", method, uri);
    
    let start = std::time::Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    println!("⬅️  {} {} - {:?}", method, uri, duration);
    
    response
}

// Aplicar middleware
let app = Router::new()
    .route("/", get(raiz))
    .layer(middleware::from_fn(logging_middleware));
```

---

## Middleware de Autenticación

```rust
use axum::{
    middleware::{self, Next},
    http::{Request, StatusCode},
    response::Response,
};

async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Obtener header Authorization
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
    
    match auth_header {
        Some(token) if token.starts_with("Bearer ") => {
            // Token válido, continuar
            Ok(next.run(request).await)
        }
        _ => {
            // No autorizado
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

// Aplicar solo a rutas protegidas
let app = Router::new()
    .route("/public", get(publico))
    .route("/private", get(privado))
    .route_layer(middleware::from_fn(auth_middleware));
```

### Rutas Protegidas Selectivas

```rust
// Rutas públicas
let public_routes = Router::new()
    .route("/", get(raiz))
    .route("/health", get(health))
    .route("/login", post(login));

// Rutas protegidas
let protected_routes = Router::new()
    .route("/users", get(listar_usuarios))
    .route("/admin", get(panel_admin))
    .layer(middleware::from_fn(auth_middleware));

// Combinar
let app = Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    .with_state(state);
```

---

## Orden de Capas

⚠️ Las capas se ejecutan en **orden inverso** al que se declaran:

```rust
let app = Router::new()
    .route("/", get(handler))
    .layer(layer_c)  // Se ejecuta 3ro (más cercano al handler)
    .layer(layer_b)  // Se ejecuta 2do
    .layer(layer_a); // Se ejecuta 1ro (más externo)

// Orden: Request -> A -> B -> C -> Handler -> C -> B -> A -> Response
```

---

## Ejemplo Completo con Estado y Capas

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Json,
    routing::{get, post, delete},
    Router,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    tareas: Arc<RwLock<HashMap<u64, Tarea>>>,
    contador: Arc<RwLock<u64>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Tarea {
    id: u64,
    titulo: String,
    completada: bool,
}

// Middleware de logging simple
async fn log_request<B>(
    req: axum::http::Request<B>,
    next: Next<B>,
) -> axum::response::Response {
    println!("📥 {} {}", req.method(), req.uri());
    next.run(req).await
}

// Handlers
async fn listar(State(state): State<AppState>) -> Json<Vec<Tarea>> {
    let tareas = state.tareas.read().await;
    Json(tareas.values().cloned().collect())
}

async fn crear(
    State(state): State<AppState>,
    Json(mut tarea): Json<Tarea>,
) -> (StatusCode, Json<Tarea>) {
    let mut contador = state.contador.write().await;
    *contador += 1;
    tarea.id = *contador;
    
    let mut tareas = state.tareas.write().await;
    tareas.insert(tarea.id, tarea.clone());
    
    (StatusCode::CREATED, Json(tarea))
}

async fn eliminar(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut tareas = state.tareas.write().await;
    if tareas.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let state = AppState {
        tareas: Arc::new(RwLock::new(HashMap::new())),
        contador: Arc::new(RwLock::new(0)),
    };

    let app = Router::new()
        .route("/tareas", get(listar).post(crear))
        .route("/tareas/:id", delete(eliminar))
        .layer(middleware::from_fn(log_request))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Servidor en http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

---

## Resumen

| Concepto | Descripción |
|----------|-------------|
| **State<T>** | Estado compartido entre handlers |
| **Arc<RwLock<T>>** | Datos mutables thread-safe |
| **Layer** | Middleware que envuelve handlers |
| **TraceLayer** | Logging automático de requests |
| **CorsLayer** | Configuración CORS |
| **from_fn** | Crear middleware desde función |

### Próximo tema

En el siguiente archivo aprenderás a estructurar un **Proyecto API REST Completo**.
