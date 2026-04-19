# 📖 Glosario - Semana 17: API REST con Axum

## A

### API (Application Programming Interface)
Interfaz que define cómo diferentes componentes de software pueden comunicarse entre sí. En este contexto, una API REST permite que clientes HTTP interactúen con el servidor.

### Async/Await
Patrón de programación asíncrona en Rust. `async` marca funciones que pueden suspenderse, y `await` espera el resultado de una operación asíncrona sin bloquear el thread.

### Axum
Framework web para Rust creado por los desarrolladores de Tokio. Diseñado para ser ergonómico, modular y compatible con el ecosistema Tower.

## C

### CORS (Cross-Origin Resource Sharing)
Mecanismo de seguridad que permite o restringe solicitudes HTTP desde dominios diferentes al del servidor.

```rust
use tower_http::cors::CorsLayer;
app.layer(CorsLayer::permissive());
```

### CRUD
Acrónimo para las cuatro operaciones básicas de persistencia:
- **C**reate: Crear (POST)
- **R**ead: Leer (GET)
- **U**pdate: Actualizar (PUT/PATCH)
- **D**elete: Eliminar (DELETE)

## D

### DTO (Data Transfer Object)
Estructura que define el formato de datos para transferencia entre cliente y servidor. Separa la representación externa de la interna.

```rust
// DTO para crear (entrada)
pub struct CrearTarea {
    pub titulo: String,
    pub descripcion: Option<String>,
}

// Entidad (interna)
pub struct Tarea {
    pub id: i64,
    pub titulo: String,
    // ...
}
```

## E

### Endpoint
URL específica de una API que responde a requests HTTP. Cada endpoint representa un recurso o acción.

```
GET  /tareas      → Listar
POST /tareas      → Crear
GET  /tareas/:id  → Obtener
PUT  /tareas/:id  → Actualizar
```

### Extractor
En Axum, componente que extrae datos de un request HTTP y los convierte a tipos Rust.

| Extractor | Fuente | Ejemplo |
|-----------|--------|---------|
| `Path<T>` | URL | `/users/:id` |
| `Query<T>` | Query string | `?page=1` |
| `Json<T>` | Body JSON | `{"name": "..."}` |
| `State<T>` | Estado compartido | Pool DB |

## H

### Handler
Función async que procesa un request HTTP y retorna una respuesta.

```rust
async fn obtener_usuario(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Usuario>, ApiError> {
    // ...
}
```

### HTTP Methods
Verbos que indican la acción a realizar:

| Método | Propósito | Idempotente |
|--------|-----------|-------------|
| GET | Obtener recurso | ✅ |
| POST | Crear recurso | ❌ |
| PUT | Reemplazar recurso | ✅ |
| PATCH | Modificar parcialmente | ❌ |
| DELETE | Eliminar recurso | ✅ |

## I

### Idempotente
Operación que produce el mismo resultado sin importar cuántas veces se ejecute. GET, PUT y DELETE son idempotentes; POST no lo es.

### IntoResponse
Trait de Axum que convierte un tipo en una respuesta HTTP.

```rust
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // Convertir error a JSON + status code
    }
}
```

## J

### JSON (JavaScript Object Notation)
Formato de intercambio de datos ligero y legible. Estándar de facto para APIs REST.

```json
{
  "id": 1,
  "titulo": "Aprender Rust",
  "completada": false
}
```

## L

### Layer
En Tower/Axum, middleware que envuelve un servicio para agregar funcionalidad.

```rust
app.layer(TraceLayer::new_for_http())
   .layer(CorsLayer::permissive())
```

## M

### Middleware
Código que se ejecuta antes/después de un handler. Útil para logging, autenticación, CORS, etc.

```rust
// Middleware de logging
async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    println!("Request: {} {}", req.method(), req.uri());
    next.run(req).await
}
```

## P

### Path Parameter
Variable en la URL que identifica un recurso específico.

```rust
// Ruta: /tareas/:id
async fn obtener(Path(id): Path<i64>) -> ...
```

### Pool (Connection Pool)
Grupo de conexiones a base de datos reutilizables. Evita el costo de crear conexiones repetidamente.

```rust
let pool = SqlitePool::connect("sqlite:tareas.db").await?;
```

## Q

### Query Parameter
Parámetros adicionales en la URL después de `?`.

```rust
// URL: /tareas?completada=true&limite=10
#[derive(Deserialize)]
struct Filtros {
    completada: Option<bool>,
    limite: Option<i32>,
}

async fn listar(Query(filtros): Query<Filtros>) -> ...
```

## R

### REST (Representational State Transfer)
Estilo arquitectónico para APIs web basado en:
- Recursos identificados por URLs
- Operaciones mediante métodos HTTP
- Representaciones (JSON, XML)
- Sin estado (stateless)

### Router
En Axum, estructura que mapea rutas a handlers.

```rust
Router::new()
    .route("/tareas", get(listar).post(crear))
    .route("/tareas/:id", get(obtener).put(actualizar).delete(eliminar))
```

## S

### Serde
Framework de serialización/deserialización en Rust. Convierte entre tipos Rust y formatos como JSON.

```rust
#[derive(Serialize, Deserialize)]
struct Tarea {
    titulo: String,
    completada: bool,
}
```

### SQLite
Base de datos relacional embebida. No requiere servidor separado, los datos se almacenan en un archivo.

### SQLx
Biblioteca Rust para interactuar con bases de datos SQL. Soporta queries verificadas en tiempo de compilación.

```rust
let tarea = sqlx::query_as::<_, Tarea>(
    "SELECT * FROM tareas WHERE id = ?"
)
.bind(id)
.fetch_one(&pool)
.await?;
```

### State
En Axum, datos compartidos entre todos los handlers. Típicamente contiene el pool de conexiones.

```rust
Router::new()
    .route("/tareas", get(listar))
    .with_state(pool)
```

### Status Code
Código numérico que indica el resultado de una request HTTP.

| Código | Significado |
|--------|-------------|
| 200 | OK |
| 201 | Created |
| 204 | No Content |
| 400 | Bad Request |
| 404 | Not Found |
| 500 | Internal Server Error |

## T

### Tokio
Runtime asíncrono para Rust. Permite ejecutar código async, manejar I/O no bloqueante y concurrencia.

```rust
#[tokio::main]
async fn main() {
    // Código async aquí
}
```

### Tower
Biblioteca de abstracciones para servicios de red. Axum está construido sobre Tower.

### Tracing
Framework de instrumentación para Rust. Permite logging estructurado con contexto.

```rust
tracing::info!("Creando tarea: {}", titulo);
```

## V

### Validación
Proceso de verificar que los datos de entrada cumplen con reglas de negocio.

```rust
if titulo.trim().is_empty() {
    return Err(ApiError::Validacion("Título vacío".into()));
}
```

---

## 📚 Referencias

- [Axum Documentation](https://docs.rs/axum)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [SQLx Documentation](https://docs.rs/sqlx)
- [HTTP Status Codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
- [REST API Best Practices](https://restfulapi.net/)
