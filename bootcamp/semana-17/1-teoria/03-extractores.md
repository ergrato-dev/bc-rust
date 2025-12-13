# 📖 Extractores

## ¿Qué son los Extractores?

Los **extractores** son tipos que implementan `FromRequest` o `FromRequestParts` y permiten extraer datos del request HTTP de forma type-safe.

```
┌─────────────────────────────────────────────────────────────┐
│                    HTTP REQUEST                             │
├─────────────────────────────────────────────────────────────┤
│  Headers      ──────────▶  HeaderMap                        │
│  URL Path     ──────────▶  Path<T>                          │
│  Query String ──────────▶  Query<T>                         │
│  Body JSON    ──────────▶  Json<T>                          │
│  Form Data    ──────────▶  Form<T>                          │
│  App State    ──────────▶  State<T>                         │
└─────────────────────────────────────────────────────────────┘
```

---

## 🎯 Diagrama Visual

![Extractores](../0-assets/03-extractores.svg)

---

## Extractores Principales

| Extractor | Fuente | Uso |
|-----------|--------|-----|
| `Path<T>` | URL path | Parámetros de ruta |
| `Query<T>` | Query string | Parámetros de consulta |
| `Json<T>` | Body | JSON del cuerpo |
| `Form<T>` | Body | Form data |
| `State<T>` | App state | Estado compartido |
| `HeaderMap` | Headers | Todos los headers |

---

## Path: Parámetros de URL

### Extracción Simple

```rust
use axum::extract::Path;

// /users/42 -> id = 42
async fn obtener_usuario(Path(id): Path<u64>) -> String {
    format!("Usuario con ID: {}", id)
}

// Registrar ruta
.route("/users/:id", get(obtener_usuario))
```

### Múltiples Parámetros

```rust
// Como tupla
async fn obtener_post(
    Path((user_id, post_id)): Path<(u64, u64)>
) -> String {
    format!("Usuario {} - Post {}", user_id, post_id)
}

// Ruta: /users/:user_id/posts/:post_id

// Como struct (más legible)
#[derive(Deserialize)]
struct PostPath {
    user_id: u64,
    post_id: u64,
}

async fn obtener_post_struct(Path(params): Path<PostPath>) -> String {
    format!("Usuario {} - Post {}", params.user_id, params.post_id)
}
```

---

## Query: Parámetros de Consulta

### Extracción de Query

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Paginacion {
    pagina: Option<u32>,
    limite: Option<u32>,
}

// /items?pagina=2&limite=10
async fn listar_items(Query(pag): Query<Paginacion>) -> String {
    let pagina = pag.pagina.unwrap_or(1);
    let limite = pag.limite.unwrap_or(20);
    format!("Página {} con {} items", pagina, limite)
}
```

### Query con Valores por Defecto

```rust
#[derive(Deserialize)]
struct Filtros {
    #[serde(default)]
    orden: String,
    
    #[serde(default = "default_limite")]
    limite: u32,
    
    activo: Option<bool>,
}

fn default_limite() -> u32 {
    10
}

// /productos?orden=precio&activo=true
async fn filtrar_productos(Query(filtros): Query<Filtros>) -> String {
    format!(
        "Orden: {}, Límite: {}, Activo: {:?}",
        filtros.orden,
        filtros.limite,
        filtros.activo
    )
}
```

---

## Json: Cuerpo JSON

### Recibir JSON

```rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CrearUsuario {
    nombre: String,
    email: String,
}

#[derive(Serialize)]
struct Usuario {
    id: u64,
    nombre: String,
    email: String,
}

async fn crear_usuario(
    Json(datos): Json<CrearUsuario>
) -> (StatusCode, Json<Usuario>) {
    let usuario = Usuario {
        id: 1,
        nombre: datos.nombre,
        email: datos.email,
    };
    (StatusCode::CREATED, Json(usuario))
}
```

### Validación Manual

```rust
#[derive(Deserialize)]
struct NuevoProducto {
    nombre: String,
    precio: f64,
}

async fn crear_producto(
    Json(producto): Json<NuevoProducto>
) -> Result<Json<Producto>, (StatusCode, String)> {
    // Validar nombre
    if producto.nombre.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Nombre es requerido".to_string()
        ));
    }
    
    // Validar precio
    if producto.precio <= 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Precio debe ser positivo".to_string()
        ));
    }
    
    Ok(Json(Producto {
        id: 1,
        nombre: producto.nombre,
        precio: producto.precio,
    }))
}
```

---

## Form: Datos de Formulario

```rust
use axum::Form;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(Form(form): Form<LoginForm>) -> String {
    format!("Login: {}", form.username)
}

// Content-Type: application/x-www-form-urlencoded
// Body: username=ana&password=secret
```

---

## Headers: Cabeceras HTTP

### HeaderMap Completo

```rust
use axum::http::HeaderMap;

async fn ver_headers(headers: HeaderMap) -> String {
    let mut resultado = String::new();
    for (nombre, valor) in headers.iter() {
        resultado.push_str(&format!(
            "{}: {}\n",
            nombre,
            valor.to_str().unwrap_or("?")
        ));
    }
    resultado
}
```

### Header Específico con TypedHeader

```rust
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};

async fn protegido(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>
) -> String {
    format!("Token: {}", auth.token())
}
```

### Header Manual

```rust
use axum::http::HeaderMap;

async fn obtener_auth(headers: HeaderMap) -> Result<String, StatusCode> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or(StatusCode::UNAUTHORIZED)
}
```

---

## Combinar Múltiples Extractores

```rust
use axum::{
    extract::{Path, Query, State},
    Json,
};

#[derive(Deserialize)]
struct FiltroQuery {
    activo: Option<bool>,
}

#[derive(Deserialize)]
struct ActualizarUsuario {
    nombre: Option<String>,
    email: Option<String>,
}

// Handler con múltiples extractores
async fn actualizar_usuario(
    State(state): State<AppState>,      // Estado de la app
    Path(id): Path<u64>,                 // ID de URL
    Query(filtro): Query<FiltroQuery>,   // Query params
    Json(datos): Json<ActualizarUsuario> // Body JSON
) -> Result<Json<Usuario>, StatusCode> {
    // Usar todos los datos extraídos
    println!("Estado: {:?}", state);
    println!("ID: {}", id);
    println!("Filtro activo: {:?}", filtro.activo);
    println!("Datos: {:?}", datos);
    
    // Lógica de actualización...
    Ok(Json(Usuario { /* ... */ }))
}
```

### Orden de Extractores

⚠️ **Importante**: Los extractores que consumen el body (`Json`, `Form`, `Bytes`) deben ir **al final**:

```rust
// ✅ Correcto - Json al final
async fn handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(body): Json<Datos>,
) { }

// ❌ Incorrecto - Json antes de Path
async fn handler(
    Json(body): Json<Datos>,
    Path(id): Path<u64>,  // Error: body ya consumido
) { }
```

---

## Extractores Opcionales

```rust
use axum::extract::OptionalPath;

// El parámetro es opcional
async fn buscar(
    path: Option<Path<String>>
) -> String {
    match path {
        Some(Path(termino)) => format!("Buscando: {}", termino),
        None => "Sin término de búsqueda".to_string(),
    }
}
```

---

## Crear Extractor Personalizado

```rust
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

// Struct para el usuario autenticado
struct UsuarioAutenticado {
    id: u64,
    rol: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for UsuarioAutenticado
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S
    ) -> Result<Self, Self::Rejection> {
        // Obtener header Authorization
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;
        
        // Validar token (simplificado)
        if auth_header.starts_with("Bearer ") {
            Ok(UsuarioAutenticado {
                id: 1,
                rol: "admin".to_string(),
            })
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

// Usar el extractor personalizado
async fn area_privada(user: UsuarioAutenticado) -> String {
    format!("Bienvenido usuario {} ({})", user.id, user.rol)
}
```

---

## Resumen de Extractores

| Extractor | Consume Body | Fuente | Ejemplo |
|-----------|--------------|--------|---------|
| `Path<T>` | No | URL | `/users/:id` |
| `Query<T>` | No | Query string | `?page=1` |
| `Json<T>` | Sí | Body JSON | `{"name": "Ana"}` |
| `Form<T>` | Sí | Body form | `name=Ana` |
| `State<T>` | No | App state | Datos compartidos |
| `HeaderMap` | No | Headers | Cabeceras HTTP |

---

## Resumen

| Concepto | Descripción |
|----------|-------------|
| **Extractor** | Tipo que extrae datos del request |
| **Path** | Parámetros de la URL |
| **Query** | Parámetros de query string |
| **Json** | Cuerpo JSON deserializado |
| **State** | Estado compartido de la app |
| **Orden** | Body extractors siempre al final |

### Próximo tema

En el siguiente archivo aprenderás sobre **Estado y Capas** para compartir datos y middleware.
