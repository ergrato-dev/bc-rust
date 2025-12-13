# 📊 Rúbrica de Evaluación - Semana 17: API REST con Axum

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **C1** | Crear servidores HTTP con Axum |
| **C2** | Implementar rutas y handlers |
| **C3** | Usar extractores para datos de request |
| **C4** | Manejar estado compartido |
| **C5** | Aplicar middleware y capas |
| **C6** | Estructurar proyecto API REST |

---

## 📋 Distribución de Puntos

| Tipo | Peso | Puntos |
|------|------|--------|
| **Conocimiento** | 30% | 30 pts |
| **Desempeño** | 40% | 40 pts |
| **Producto** | 30% | 30 pts |
| **Total** | 100% | 100 pts |

---

## 🧠 Evaluación de Conocimiento (30 pts)

### Preguntas Teóricas (15 pts)

| # | Pregunta | Pts |
|---|----------|-----|
| 1 | ¿Qué es un Router en Axum? | 3 |
| 2 | ¿Cuál es la diferencia entre `get()` y `post()` en rutas? | 3 |
| 3 | ¿Para qué sirve `Json<T>` como extractor? | 3 |
| 4 | ¿Cómo se comparte estado entre handlers? | 3 |
| 5 | ¿Qué son los códigos de estado HTTP y cuándo usar cada uno? | 3 |

### Código Conceptual (15 pts)

#### Pregunta 1 (5 pts): Identificar el error

```rust
async fn crear_usuario(usuario: Usuario) -> Json<Usuario> {
    Json(usuario)
}
```

**Respuesta esperada**: Falta el extractor `Json<>` en el parámetro: `Json(usuario): Json<Usuario>`

#### Pregunta 2 (5 pts): Completar el handler

```rust
async fn obtener_usuario(
    // ¿Qué extractor usar?
    State(state): State<AppState>,
    // ¿Cómo extraer el ID de la ruta?
) -> Result<Json<Usuario>, StatusCode> {
    // ...
}
```

**Respuesta esperada**: `Path(id): Path<u64>`

#### Pregunta 3 (5 pts): Explicar el código

```rust
let app = Router::new()
    .route("/users", get(listar).post(crear))
    .route("/users/:id", get(obtener).delete(eliminar))
    .with_state(state);
```

**Respuesta esperada**: Router con dos rutas, la primera maneja GET y POST para `/users`, la segunda GET y DELETE para `/users/:id` con parámetro dinámico, todas con acceso al estado compartido.

---

## 💻 Evaluación de Desempeño (40 pts)

### Práctica 1: Hello Axum (8 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| Servidor arranca | 2 | `cargo run` sin errores |
| Ruta raíz funciona | 2 | GET `/` retorna respuesta |
| Ruta saludo con parámetro | 2 | `/saludo/:nombre` funciona |
| Formato código | 2 | `cargo fmt` y `cargo clippy` |

### Práctica 2: CRUD Usuarios (10 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| GET listar | 2 | `/users` retorna lista |
| POST crear | 2 | Crea y retorna usuario |
| GET por ID | 2 | `/users/:id` funciona |
| DELETE eliminar | 2 | Elimina correctamente |
| Estado compartido | 2 | Datos persisten entre requests |

### Práctica 3: Validación (10 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| Validar campos requeridos | 3 | Retorna 400 si faltan |
| Validar formato email | 2 | Regex o validación básica |
| Mensajes de error JSON | 3 | Estructura de error clara |
| Códigos HTTP correctos | 2 | 400, 404, etc. apropiados |

### Práctica 4: Middleware (12 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| Logging de requests | 3 | Muestra método, ruta, tiempo |
| Middleware de autenticación | 4 | Verifica header/token |
| Manejo de errores global | 3 | Captura y formatea errores |
| Composición de capas | 2 | Tower layers correctas |

---

## 🏆 Evaluación de Producto (30 pts)

### Proyecto: API Tareas

#### Funcionalidad (15 pts)

| Endpoint | Pts | Verificación |
|----------|-----|--------------|
| GET `/tareas` | 2 | Lista todas las tareas |
| GET `/tareas/:id` | 2 | Retorna tarea o 404 |
| POST `/tareas` | 3 | Crea tarea con validación |
| PUT `/tareas/:id` | 3 | Actualiza tarea existente |
| DELETE `/tareas/:id` | 2 | Elimina y retorna 204 |
| Filtrado por estado | 3 | Query param `?estado=` |

#### Calidad del Código (10 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| Estructura modular | 3 | Separación routes/handlers/models |
| Manejo de errores | 3 | Result con errores descriptivos |
| Documentación | 2 | Comentarios en handlers |
| Formato y linting | 2 | fmt + clippy sin warnings |

#### Tests (5 pts)

| Criterio | Pts | Descripción |
|----------|-----|-------------|
| Tests de integración | 3 | Al menos 5 tests de API |
| Cobertura CRUD | 2 | Tests para cada operación |

---

## 📝 Escala de Calificación

| Rango | Nota | Descripción |
|-------|------|-------------|
| 90-100 | A | Excelente - Dominio completo |
| 80-89 | B | Bueno - Dominio sólido |
| 70-79 | C | Satisfactorio - Comprensión básica |
| 60-69 | D | Mínimo - Necesita refuerzo |
| 0-59 | F | Insuficiente - No aprobado |

---

## ✅ Checklist de Entrega

```
□ Servidor arranca en puerto 3000
□ Todos los endpoints CRUD funcionan
□ Validación de datos implementada
□ Códigos HTTP correctos
□ Estado compartido funciona
□ Tests de integración pasan
□ cargo fmt aplicado
□ cargo clippy sin warnings
□ README con instrucciones
```

---

## 🎯 Criterios de Aprobación

Para aprobar esta semana se requiere:

1. **Mínimo 60 puntos** en total
2. **Al menos 50%** en cada categoría:
   - Conocimiento: mínimo 15/30
   - Desempeño: mínimo 20/40
   - Producto: mínimo 15/30
3. **Proyecto funcional**: Al menos GET y POST funcionando
4. **Tests pasando**: `cargo test` sin errores

---

## 🏁 Certificación del Bootcamp

Al aprobar esta semana final, el estudiante recibe:

- ✅ Certificado de completación del Bootcamp
- ✅ Proyecto final como portfolio
- ✅ Acceso a recursos avanzados
- ✅ Membresía comunidad Rustaceans

**¡Felicitaciones por completar Rust: Zero to Hero!** 🦀
