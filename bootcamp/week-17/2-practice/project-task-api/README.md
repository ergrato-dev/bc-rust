# 📝 Task API Project

Bootcamp final project: Complete REST API with SQLite for task management.

## 🎯 Objectives

- Build a functional REST API
- SQLite persistence
- Data validation
- Robust error handling
- Integration tests
- **OpenAPI documentation with Swagger UI**

---

## 📚 API Documentation

The API includes interactive documentation with **Swagger UI**:

- **Swagger UI**: http://localhost:3000/swagger-ui
- **OpenAPI JSON**: http://localhost:3000/api-docs/openapi.json

### Features

- Auto-generated documentation with `utoipa`
- Interactive interface to test endpoints
- Documented request/response schemas
- Examples included for each endpoint

---

## 🏗️ Architecture

```
proyecto-api-tareas/
├── Cargo.toml
├── src/
│   ├── main.rs        # Entry point + OpenAPI
│   ├── lib.rs         # Module exports
│   ├── db.rs          # SQLite Pool
│   ├── error.rs       # Error types
│   ├── models.rs      # Structs + ToSchema
│   ├── handlers.rs    # Handlers + utoipa::path
│   └── routes.rs      # Route definitions
└── tests/
    └── api_tests.rs   # Integration tests
```

---

## 📊 Endpoints

| Method | Route          | Description          |
| ------ | -------------- | -------------------- |
| GET    | /tasks         | List all             |
| POST   | /tasks         | Create new task      |
| GET    | /tasks/:id     | Get by ID            |
| PUT    | /tasks/:id     | Update task          |
| DELETE | /tasks/:id     | Delete task          |
| GET    | /tasks/stats   | Statistics           |
| GET    | /swagger-ui    | 📚 Documentation     |

### 🔍 Filters (Query Parameters)

| Parameter  | Type   | Description             |
| ---------- | ------ | ----------------------- |
| completed  | bool   | Filter by status        |
| limit      | int    | Maximum results         |
| offset     | int    | Skip N results          |

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

### Create a task

```bash
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Rust",
    "description": "Complete the bootcamp"
  }'
```

**Response (201 Created):**
```json
{
  "id": 1,
  "title": "Learn Rust",
  "description": "Complete the bootcamp",
  "completed": false,
  "created_at": "2025-01-15T10:30:00Z",
  "updated_at": null
}
```

### List all tasks

```bash
curl http://localhost:3000/tasks
```

### List only pending

```bash
curl "http://localhost:3000/tasks?completed=false"
```

### List with pagination

```bash
curl "http://localhost:3000/tasks?limit=10&offset=0"
```

### Get task by ID

```bash
curl http://localhost:3000/tasks/1
```

### Update a task

```bash
curl -X PUT http://localhost:3000/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Advanced Rust",
    "completed": true
  }'
```

### Delete a task

```bash
curl -X DELETE http://localhost:3000/tasks/1
```

### View statistics

```bash
curl http://localhost:3000/tasks/stats
```

**Response:**
```json
{
  "total": 10,
  "completed": 3,
  "pending": 7
}
```

---

## ✅ Tests

### Run all tests

```bash
# With Docker Compose
docker compose run --rm rust-dev \
    cargo test -p proyecto-api-tareas

# Local
cargo test -p proyecto-api-tareas
```

### Run specific test

```bash
cargo test -p proyecto-api-tareas test_create_task_success
```

### Show test output

```bash
cargo test -p proyecto-api-tareas -- --nocapture
```

---

## 🗄️ Database

The project uses SQLite with file `tasks.db` created automatically.

### Schema

```sql
CREATE TABLE IF NOT EXISTS tareas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    titulo TEXT NOT NULL,
    descripcion TEXT,
    completada BOOLEAN NOT NULL DEFAULT FALSE,
    creada_en DATETIME DEFAULT CURRENT_TIMESTAMP,
    actualizada_en DATETIME
);
```

### Location

- **Docker**: `/workspace/tasks.db`
- **Local**: Execution directory

---

## 🔧 Dependencies

| Crate               | Version | Purpose                  |
| ------------------- | ------- | ------------------------ |
| axum                | 0.8     | Web framework            |
| tokio               | 1       | Async runtime            |
| sqlx                | 0.8     | SQLite database          |
| serde               | 1       | JSON serialization       |
| tower-http          | 0.6     | Middleware (CORS, trace) |
| tracing             | 0.1     | Logging                  |
| thiserror           | 2       | Typed errors             |
| **utoipa**          | **5**   | **OpenAPI/Swagger**      |
| **utoipa-swagger-ui** | **9** | **Swagger UI**           |

---

## 📁 Code Structure

### `db.rs` - Database Connection

```rust
pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    // Creates connection pool and table if not exists
}
```

### `models.rs` - Data Structures

```rust
pub struct Task { ... }           // Main entity
pub struct CreateTask { ... }      // DTO for creation
pub struct UpdateTask { ... }      // DTO for update
pub struct TaskFilters { ... }     // Query parameters
pub struct TaskStats { ... }       // Stats response
```

### `handlers.rs` - Business Logic

```rust
pub async fn list_tasks(...) -> Result<Json<Vec<Task>>, ApiError>
pub async fn create_task(...) -> Result<(StatusCode, Json<Task>), ApiError>
pub async fn get_task(...) -> Result<Json<Task>, ApiError>
pub async fn update_task(...) -> Result<Json<Task>, ApiError>
pub async fn delete_task(...) -> Result<StatusCode, ApiError>
pub async fn get_stats(...) -> Result<Json<TaskStats>, ApiError>
```

### `error.rs` - Error Handling

```rust
pub enum ApiError {
    NotFound(String),
    Validation(String),
    Database(String),
    Internal(String),
}
```

---

## 🎓 Applied Concepts

| Week | Concept                  | Application                     |
| ---- | ------------------------ | ------------------------------- |
| 2    | Ownership                | Pool shared with Arc            |
| 5    | Error Handling           | Result, thiserror, ApiError     |
| 8    | Traits                   | FromRow, Serialize, Deserialize |
| 9    | Generics                 | Json<T>, State<T>               |
| 11   | Closures                 | Async handlers                  |
| 13   | Concurrency              | Tokio tasks                     |
| 14   | Async/Await              | Entire project                  |
| 15   | Testing                  | Integration tests               |

---

## 📋 Evaluation Rubric

See [RUBRICA_EVALUACION.md](../../RUBRICA_EVALUACION.md) for project evaluation criteria.

---

## 🚀 Suggested Extensions

1. **Authentication**: JWT with headers
2. **Categories**: Many-to-many relationship
3. **Priorities**: Enum with ordering
4. **Deadlines**: Deadline field
5. **Search**: Full-text search
6. **Export**: Generate CSV/JSON

---

**Congratulations on completing the bootcamp! 🦀🎉**
