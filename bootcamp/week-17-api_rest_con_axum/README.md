# 🦀 Semana 17: API REST con Axum

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 17 de 17 |
| **Tema** | API REST con Axum |
| **Duración** | 4 horas |
| **Nivel** | Avanzado |
| **Requisitos** | Semana 16 (Testing), Semana 15 (Async) |

---

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. **Comprender** la arquitectura de APIs REST en Rust
2. **Usar** el framework Axum para crear servidores HTTP
3. **Implementar** rutas, handlers y extractores
4. **Manejar** estado compartido con `State`
5. **Aplicar** middleware para logging y autenticación
6. **Estructurar** un proyecto API REST completo

---

## 📚 Contenido Teórico

### Fundamentos de API REST

```
┌─────────────────────────────────────────────────────────────┐
│                    ARQUITECTURA REST                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   Cliente HTTP                    Servidor Axum             │
│   ┌─────────────┐                ┌─────────────────┐       │
│   │   Request   │───────────────▶│     Router      │       │
│   │  GET /users │                │                 │       │
│   └─────────────┘                │   ┌─────────┐   │       │
│                                  │   │ Handler │   │       │
│   ┌─────────────┐                │   └────┬────┘   │       │
│   │  Response   │◀───────────────│        │        │       │
│   │  JSON [...]  │               │   ┌────▼────┐   │       │
│   └─────────────┘                │   │  State  │   │       │
│                                  │   └─────────┘   │       │
│                                  └─────────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

### Métodos HTTP

| Método | Operación | Descripción |
|--------|-----------|-------------|
| `GET` | Read | Obtener recursos |
| `POST` | Create | Crear nuevo recurso |
| `PUT` | Update | Actualizar recurso completo |
| `PATCH` | Update | Actualizar parcialmente |
| `DELETE` | Delete | Eliminar recurso |

### Códigos de Estado

| Código | Significado |
|--------|-------------|
| `200` | OK - Éxito |
| `201` | Created - Recurso creado |
| `400` | Bad Request - Error del cliente |
| `404` | Not Found - No encontrado |
| `500` | Internal Server Error |

---

## 📖 Material de Estudio

### Teoría

| # | Tema | Archivo | Diagrama |
|---|------|---------|----------|
| 1 | Introducción a Axum | [01-intro-axum.md](1-teoria/01-intro-axum.md) | [SVG](0-assets/01-intro-axum.svg) |
| 2 | Rutas y Handlers | [02-rutas-handlers.md](1-teoria/02-rutas-handlers.md) | [SVG](0-assets/02-rutas-handlers.svg) |
| 3 | Extractores | [03-extractores.md](1-teoria/03-extractores.md) | [SVG](0-assets/03-extractores.svg) |
| 4 | Estado y Capas | [04-estado-capas.md](1-teoria/04-estado-capas.md) | [SVG](0-assets/04-estado-capas.svg) |
| 5 | Proyecto Completo | [05-proyecto-completo.md](1-teoria/05-proyecto-completo.md) | [SVG](0-assets/05-proyecto-completo.svg) |

### Práctica

| # | Ejercicio | Descripción | Dificultad |
|---|-----------|-------------|------------|
| 1 | [Hello Axum](2-practica/practica-01-hello-axum/) | Primer servidor HTTP | ⭐ |
| 2 | [CRUD Usuarios](2-practica/practica-02-crud/) | API CRUD completa | ⭐⭐ |
| 3 | [Validación](2-practica/practica-03-validacion/) | Validar requests | ⭐⭐ |
| 4 | [Middleware](2-practica/practica-04-middleware/) | Logging y auth | ⭐⭐⭐ |
| 🏆 | [API Tareas](2-practica/proyecto-api-tareas/) | Proyecto final completo | ⭐⭐⭐ |

---

## 🚀 Inicio Rápido

### Dependencias en Cargo.toml

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.6", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

### Servidor Mínimo

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "¡Hola, Axum!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Servidor en http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

---

## 🔧 Comandos Docker

```bash
# Ejecutar servidor
docker compose run --rm -p 3000:3000 rust-dev cargo run -p proyecto-api-tareas

# Ejecutar tests
docker compose run --rm rust-dev cargo test -p proyecto-api-tareas

# Verificar código
docker compose run --rm rust-dev cargo clippy -p proyecto-api-tareas
```

### Probar con curl

```bash
# GET todos los usuarios
curl http://localhost:3000/users

# POST crear usuario
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"nombre": "Ana", "email": "ana@test.com"}'

# GET usuario por ID
curl http://localhost:3000/users/1

# DELETE eliminar usuario
curl -X DELETE http://localhost:3000/users/1
```

---

## 📊 Estructura del Proyecto API

```
proyecto-api-tareas/
├── Cargo.toml
├── src/
│   ├── main.rs          # Punto de entrada
│   ├── lib.rs           # Módulos públicos
│   ├── routes/          # Definición de rutas
│   │   ├── mod.rs
│   │   └── tareas.rs
│   ├── handlers/        # Lógica de handlers
│   │   ├── mod.rs
│   │   └── tareas.rs
│   ├── models/          # Estructuras de datos
│   │   ├── mod.rs
│   │   └── tarea.rs
│   └── state.rs         # Estado compartido
├── tests/
│   └── api_tests.rs     # Tests de integración
└── README.md
```

---

## 📈 Criterios de Evaluación

| Criterio | Peso | Descripción |
|----------|------|-------------|
| **Conocimiento** | 30% | Comprensión de REST y Axum |
| **Desempeño** | 40% | Ejercicios completados |
| **Producto** | 30% | API funcional con tests |

### Checklist de Entrega

- [ ] Servidor arranca sin errores
- [ ] Rutas CRUD implementadas
- [ ] Manejo de errores con códigos HTTP correctos
- [ ] Tests de integración pasan
- [ ] Código formateado (`cargo fmt`)
- [ ] Sin warnings de clippy

---

## 🎓 Proyecto Final del Bootcamp

Esta semana construirás una **API REST de gestión de tareas** que incluye:

1. **CRUD completo** de tareas
2. **Filtrado** por estado (pendiente/completada)
3. **Validación** de datos de entrada
4. **Manejo de errores** con respuestas JSON
5. **Tests de integración** con `axum::test`
6. **Documentación** de endpoints

### Endpoints del Proyecto

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/tareas` | Listar todas las tareas |
| GET | `/tareas/:id` | Obtener una tarea |
| POST | `/tareas` | Crear nueva tarea |
| PUT | `/tareas/:id` | Actualizar tarea |
| DELETE | `/tareas/:id` | Eliminar tarea |
| GET | `/tareas?estado=pendiente` | Filtrar por estado |

---

## 🔗 Recursos

- [Documentación Axum](https://docs.rs/axum/latest/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Tower Middleware](https://docs.rs/tower/latest/tower/)
- [Serde JSON](https://docs.rs/serde_json/latest/serde_json/)

---

## 🏁 ¡Felicitaciones!

Al completar esta semana, habrás terminado el **Bootcamp Rust: Zero to Hero**.

Durante 17 semanas (68 horas) has aprendido:

- ✅ Fundamentos de Rust
- ✅ Sistema de Ownership
- ✅ Structs, Enums y Pattern Matching
- ✅ Error Handling con Result/Option
- ✅ Traits y Generics
- ✅ Lifetimes
- ✅ Closures e Iteradores
- ✅ Smart Pointers
- ✅ Concurrencia
- ✅ Async/Await
- ✅ Testing y Documentación
- ✅ **APIs REST con Axum**

**¡Ahora eres un Rustacean! 🦀**
