# 📝 Proyecto API de Tareas

Proyecto final del bootcamp: API REST completa con SQLite para gestión de tareas.

## 🎯 Objetivos

- Construir una API REST funcional
- Persistencia con SQLite
- Validación de datos
- Manejo de errores robusto
- Tests de integración

---

## 🏗️ Arquitectura

```
proyecto-api-tareas/
├── Cargo.toml
├── src/
│   ├── main.rs        # Punto de entrada
│   ├── lib.rs         # Exports de módulos
│   ├── db.rs          # Pool SQLite
│   ├── error.rs       # Tipos de error
│   ├── models.rs      # Structs de datos
│   ├── handlers.rs    # Lógica de endpoints
│   └── routes.rs      # Definición de rutas
└── tests/
    └── api_tests.rs   # Tests de integración
```

---

## 📊 Endpoints

| Método | Ruta                   | Descripción           |
| ------ | ---------------------- | --------------------- |
| GET    | /tareas                | Listar todas          |
| POST   | /tareas                | Crear nueva tarea     |
| GET    | /tareas/:id            | Obtener por ID        |
| PUT    | /tareas/:id            | Actualizar tarea      |
| DELETE | /tareas/:id            | Eliminar tarea        |
| GET    | /tareas/estadisticas   | Estadísticas          |

### 🔍 Filtros (Query Parameters)

| Parámetro   | Tipo   | Descripción               |
| ----------- | ------ | ------------------------- |
| completada  | bool   | Filtrar por estado        |
| limite      | int    | Máximo de resultados      |
| offset      | int    | Saltar N resultados       |

---

## 🚀 Ejecución

### Con Docker Compose

```bash
# Desde la raíz del proyecto
docker compose run --rm -p 3000:3000 rust-dev \
    cargo run -p proyecto-api-tareas
```

### Local

```bash
cargo run -p proyecto-api-tareas
```

El servidor iniciará en `http://localhost:3000`.

---

## 📝 Ejemplos de Uso

### Crear una tarea

```bash
curl -X POST http://localhost:3000/tareas \
  -H "Content-Type: application/json" \
  -d '{
    "titulo": "Aprender Rust",
    "descripcion": "Completar el bootcamp"
  }'
```

**Respuesta (201 Created):**
```json
{
  "id": 1,
  "titulo": "Aprender Rust",
  "descripcion": "Completar el bootcamp",
  "completada": false,
  "fecha_creacion": "2025-01-15T10:30:00Z",
  "fecha_actualizacion": null
}
```

### Listar todas las tareas

```bash
curl http://localhost:3000/tareas
```

### Listar solo pendientes

```bash
curl "http://localhost:3000/tareas?completada=false"
```

### Listar con paginación

```bash
curl "http://localhost:3000/tareas?limite=10&offset=0"
```

### Obtener tarea por ID

```bash
curl http://localhost:3000/tareas/1
```

### Actualizar una tarea

```bash
curl -X PUT http://localhost:3000/tareas/1 \
  -H "Content-Type: application/json" \
  -d '{
    "titulo": "Aprender Rust Avanzado",
    "completada": true
  }'
```

### Eliminar una tarea

```bash
curl -X DELETE http://localhost:3000/tareas/1
```

### Ver estadísticas

```bash
curl http://localhost:3000/tareas/estadisticas
```

**Respuesta:**
```json
{
  "total": 10,
  "completadas": 3,
  "pendientes": 7
}
```

---

## ✅ Tests

### Ejecutar todos los tests

```bash
# Con Docker Compose
docker compose run --rm rust-dev \
    cargo test -p proyecto-api-tareas

# Local
cargo test -p proyecto-api-tareas
```

### Ejecutar un test específico

```bash
cargo test -p proyecto-api-tareas test_crear_tarea_exitoso
```

### Ver output de tests

```bash
cargo test -p proyecto-api-tareas -- --nocapture
```

---

## 🗄️ Base de Datos

El proyecto usa SQLite con archivo `tareas.db` creado automáticamente.

### Esquema

```sql
CREATE TABLE IF NOT EXISTS tareas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    titulo TEXT NOT NULL,
    descripcion TEXT,
    completada BOOLEAN NOT NULL DEFAULT FALSE,
    fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion DATETIME
);
```

### Ubicación

- **Docker**: `/workspace/tareas.db`
- **Local**: Directorio de ejecución

---

## 🔧 Dependencias

| Crate               | Versión | Propósito                |
| ------------------- | ------- | ------------------------ |
| axum                | 0.8     | Web framework            |
| tokio               | 1       | Runtime async            |
| sqlx                | 0.8     | Base de datos SQLite     |
| serde               | 1       | Serialización JSON       |
| tower-http          | 0.6     | Middleware (CORS, trace) |
| tracing             | 0.1     | Logging                  |
| thiserror           | 2       | Errores tipados          |

---

## 📁 Estructura de Código

### `db.rs` - Conexión a Base de Datos

```rust
pub async fn crear_pool() -> Result<SqlitePool, sqlx::Error> {
    // Crea pool de conexiones y tabla si no existe
}
```

### `models.rs` - Estructuras de Datos

```rust
pub struct Tarea { ... }           // Entidad principal
pub struct CrearTarea { ... }       // DTO para crear
pub struct ActualizarTarea { ... }  // DTO para actualizar
pub struct FiltroTareas { ... }     // Query parameters
pub struct EstadisticasTareas { ... } // Respuesta stats
```

### `handlers.rs` - Lógica de Negocio

```rust
pub async fn listar(...) -> Result<Json<Vec<Tarea>>, ApiError>
pub async fn crear(...) -> Result<(StatusCode, Json<Tarea>), ApiError>
pub async fn obtener(...) -> Result<Json<Tarea>, ApiError>
pub async fn actualizar(...) -> Result<Json<Tarea>, ApiError>
pub async fn eliminar(...) -> Result<StatusCode, ApiError>
pub async fn estadisticas(...) -> Result<Json<EstadisticasTareas>, ApiError>
```

### `error.rs` - Manejo de Errores

```rust
pub enum ApiError {
    NotFound(String),
    Validacion(String),
    Database(String),
    Interno(String),
}
```

---

## 🎓 Conceptos Aplicados

| Semana | Concepto                 | Aplicación                      |
| ------ | ------------------------ | ------------------------------- |
| 2      | Ownership                | Pool compartido con Arc         |
| 5      | Error Handling           | Result, thiserror, ApiError     |
| 8      | Traits                   | FromRow, Serialize, Deserialize |
| 9      | Generics                 | Json<T>, State<T>               |
| 11     | Closures                 | Handlers async                  |
| 13     | Concurrencia             | Tokio tasks                     |
| 14     | Async/Await              | Todo el proyecto                |
| 15     | Testing                  | Integration tests               |

---

## 📋 Rúbrica de Evaluación

Ver [RUBRICA_EVALUACION.md](../../RUBRICA_EVALUACION.md) para los criterios de evaluación del proyecto.

---

## 🚀 Extensiones Sugeridas

1. **Autenticación**: JWT con headers
2. **Categorías**: Relación many-to-many
3. **Prioridades**: Enum con ordenamiento
4. **Fechas límite**: Campo deadline
5. **Búsqueda**: Full-text search
6. **Export**: Generar CSV/JSON

---

**¡Felicitaciones por completar el bootcamp! 🦀🎉**
