# 📚 Recursos - Semana 17: API REST con Axum

## 📖 Documentación Oficial

### Axum
- **Docs.rs**: https://docs.rs/axum/latest/axum/
- **GitHub**: https://github.com/tokio-rs/axum
- **Examples**: https://github.com/tokio-rs/axum/tree/main/examples

### Tokio
- **Website**: https://tokio.rs/
- **Tutorial**: https://tokio.rs/tokio/tutorial
- **API Reference**: https://docs.rs/tokio/latest/tokio/

### SQLx
- **Docs.rs**: https://docs.rs/sqlx/latest/sqlx/
- **GitHub**: https://github.com/launchbadge/sqlx
- **SQLite Guide**: https://docs.rs/sqlx/latest/sqlx/sqlite/index.html

### Tower
- **Docs.rs**: https://docs.rs/tower/latest/tower/
- **tower-http**: https://docs.rs/tower-http/latest/tower_http/

### Serde
- **Website**: https://serde.rs/
- **Docs.rs**: https://docs.rs/serde/latest/serde/

---

## 📺 Videos Recomendados

### Axum Fundamentals
- **"Rust Axum Full Course"** - Jeremy Chone
  - https://www.youtube.com/watch?v=XZtlD_m59sM
  - Curso completo de Axum con ejemplos prácticos

- **"Building APIs in Rust with Axum"** - Dreams of Code
  - https://www.youtube.com/watch?v=T_wgAkBikS4
  - Tutorial paso a paso para principiantes

### Tokio y Async
- **"Async Rust"** - Jon Gjengset
  - https://www.youtube.com/watch?v=ThjvMReOXYM
  - Explicación profunda de async/await

### SQLx
- **"SQLx Tutorial"** - Let's Get Rusty
  - https://www.youtube.com/watch?v=Cbw1EPWrmfA
  - Introducción a SQLx con SQLite

---

## 📝 Artículos y Tutoriales

### Axum
1. **"Zero to Production in Rust"** (Capítulos sobre APIs)
   - https://www.zero2prod.com/
   - Libro completo sobre APIs en Rust (usa Actix, pero conceptos aplicables)

2. **"Axum Tutorial"** - tokio.rs
   - https://tokio.rs/blog/2021-08-axum
   - Post oficial de introducción

3. **"Building a REST API with Axum"** - LogRocket
   - https://blog.logrocket.com/rust-axum-rest-api/
   - Tutorial práctico completo

### Error Handling
4. **"Error Handling in Rust"** - The Rust Book
   - https://doc.rust-lang.org/book/ch09-00-error-handling.html
   - Fundamentos de manejo de errores

5. **"thiserror vs anyhow"** - Nick Cameron
   - https://nick.groenen.me/posts/rust-error-handling/
   - Cuándo usar cada biblioteca

### SQLite
6. **"SQLite in Rust"** - SQLx README
   - https://github.com/launchbadge/sqlx#sqlite
   - Guía rápida de SQLx con SQLite

---

## 🛠️ Herramientas de Desarrollo

### Testing de APIs
- **curl**: Herramienta de línea de comandos (incluida en el contenedor)
- **httpie**: Cliente HTTP amigable - https://httpie.io/
- **Postman**: GUI para testing de APIs - https://www.postman.com/
- **Insomnia**: Alternativa a Postman - https://insomnia.rest/
- **REST Client** (VS Code): Extensión para probar APIs desde el editor

### Debugging
- **rust-analyzer**: LSP para VS Code
- **tracing-subscriber**: Logging estructurado
- **tokio-console**: Debugger para aplicaciones async

### Base de Datos
- **DB Browser for SQLite**: GUI para SQLite - https://sqlitebrowser.org/
- **SQLite CLI**: Herramienta de línea de comandos

---

## 📦 Crates Relacionados

### Web Frameworks (Alternativas)
| Crate | Descripción |
|-------|-------------|
| `actix-web` | Framework maduro y rápido |
| `rocket` | Framework con macros ergonómicas |
| `warp` | Framework funcional basado en filters |
| `poem` | Framework simple y extensible |

### Middleware y Utilidades
| Crate | Descripción |
|-------|-------------|
| `tower-http` | Middleware HTTP (cors, trace, compression) |
| `axum-extra` | Extractors y utilidades adicionales |
| `validator` | Validación de structs con macros |
| `garde` | Alternativa moderna a validator |

### Autenticación
| Crate | Descripción |
|-------|-------------|
| `jsonwebtoken` | Encode/decode JWT |
| `axum-login` | Sesiones y autenticación |
| `oauth2` | Cliente OAuth2 |
| `argon2` | Hash de contraseñas |

### Base de Datos
| Crate | Descripción |
|-------|-------------|
| `sqlx` | Queries SQL async (SQLite, Postgres, MySQL) |
| `diesel` | ORM con queries type-safe |
| `sea-orm` | ORM async inspirado en ActiveRecord |
| `rusqlite` | Bindings SQLite síncronos |

### Serialización
| Crate | Descripción |
|-------|-------------|
| `serde` | Framework de serialización |
| `serde_json` | JSON support |
| `chrono` | Fechas y timestamps |
| `uuid` | UUIDs |

### Logging y Tracing
| Crate | Descripción |
|-------|-------------|
| `tracing` | Instrumentación estructurada |
| `tracing-subscriber` | Recolección de traces |
| `env_logger` | Logger simple basado en env vars |

---

## 🎯 Ejercicios Adicionales

### Nivel Básico
1. Agregar endpoint de health check (`GET /health`)
2. Implementar ordenamiento por fecha
3. Agregar campo de prioridad (alta, media, baja)

### Nivel Intermedio
4. Implementar búsqueda por título (`?q=texto`)
5. Agregar categorías (relación one-to-many)
6. Implementar soft delete (campo `eliminado`)

### Nivel Avanzado
7. Agregar autenticación con JWT
8. Implementar rate limiting
9. Agregar websockets para actualizaciones en tiempo real
10. Containerizar con Docker y deploy

---

## 🔗 Proyectos de Ejemplo

### Repositorios de Referencia
1. **realworld-axum-sqlx**
   - https://github.com/davidpdrsn/realworld-axum-sqlx
   - Implementación completa del spec RealWorld

2. **axum-login example**
   - https://github.com/maxcountryman/axum-login/tree/main/examples
   - Ejemplos de autenticación

3. **shuttle-examples**
   - https://github.com/shuttle-hq/shuttle-examples
   - Proyectos Axum deployables

---

## 📋 Cheatsheets

### Axum Extractors Quick Reference

```rust
// Path parameter
async fn handler(Path(id): Path<i64>) -> impl IntoResponse { }

// Query parameters
async fn handler(Query(params): Query<Params>) -> impl IntoResponse { }

// JSON body
async fn handler(Json(body): Json<Input>) -> impl IntoResponse { }

// State
async fn handler(State(pool): State<Pool>) -> impl IntoResponse { }

// Headers
async fn handler(headers: HeaderMap) -> impl IntoResponse { }

// Multiple extractors
async fn handler(
    State(pool): State<Pool>,
    Path(id): Path<i64>,
    Json(body): Json<Input>,
) -> impl IntoResponse { }
```

### HTTP Status Codes Quick Reference

```rust
use axum::http::StatusCode;

// Success
StatusCode::OK              // 200
StatusCode::CREATED         // 201
StatusCode::NO_CONTENT      // 204

// Client Errors
StatusCode::BAD_REQUEST     // 400
StatusCode::UNAUTHORIZED    // 401
StatusCode::FORBIDDEN       // 403
StatusCode::NOT_FOUND       // 404
StatusCode::CONFLICT        // 409

// Server Errors
StatusCode::INTERNAL_SERVER_ERROR  // 500
StatusCode::SERVICE_UNAVAILABLE    // 503
```

### SQLx Quick Reference

```rust
// Query one
let row = sqlx::query_as::<_, Tarea>("SELECT * FROM tareas WHERE id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await?;

// Query optional
let row = sqlx::query_as::<_, Tarea>("SELECT * FROM tareas WHERE id = ?")
    .bind(id)
    .fetch_optional(&pool)
    .await?;

// Query all
let rows = sqlx::query_as::<_, Tarea>("SELECT * FROM tareas")
    .fetch_all(&pool)
    .await?;

// Insert with returning
let row = sqlx::query_as::<_, Tarea>(
    "INSERT INTO tareas (titulo) VALUES (?) RETURNING *"
)
    .bind(&titulo)
    .fetch_one(&pool)
    .await?;

// Update
sqlx::query("UPDATE tareas SET titulo = ? WHERE id = ?")
    .bind(&titulo)
    .bind(id)
    .execute(&pool)
    .await?;

// Delete
sqlx::query("DELETE FROM tareas WHERE id = ?")
    .bind(id)
    .execute(&pool)
    .await?;
```

---

## 🎓 Próximos Pasos

Después de completar esta semana, considera explorar:

1. **Deploy**: Shuttle, Fly.io, Railway
2. **Frontend**: Yew, Leptos, Dioxus
3. **GraphQL**: async-graphql
4. **gRPC**: tonic
5. **WebSockets**: axum websocket support
6. **Microservicios**: Message queues, service mesh

---

**¡Felicitaciones por completar el bootcamp de Rust! 🦀🎉**
