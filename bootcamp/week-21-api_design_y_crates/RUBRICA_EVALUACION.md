# 📊 Rúbrica de Evaluación — Semana 21: Diseño de APIs

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **C1** | Aplicar Rust API Guidelines en el diseño de una librería |
| **C2** | Implementar el patrón Builder con validación |
| **C3** | Escribir doctests funcionales y ejemplos en `examples/` |
| **C4** | Diseñar tipos de error custom con `thiserror` |
| **C5** | Gestionar semver y CHANGELOG correctamente |
| **C6** | Preparar un crate para publicación (metadata, docs, tests) |

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

1. ¿Cuáles son las 3 categorías de cambios en semver? (5 pts)
2. ¿Por qué es mejor `impl Into<String>` que `String` como parámetro? (5 pts)
3. ¿Cómo funciona `#[deny(missing_docs)]`? (5 pts)
4. ¿Cuándo usar `thiserror` vs `anyhow`? (5 pts)
5. ¿Qué son los "newtype patterns" y qué problema resuelven? (5 pts)
6. ¿Qué metadata es obligatoria en `Cargo.toml` para publicar? (5 pts)

---

## ⚙️ Evaluación de Desempeño (40 pts)

### Diseñar una API ergonómica (20 pts)

Rediseñar esta API poco ergonómica:
```rust
// Antes (no ergonómico)
fn create_user(name: String, age: u32, email: String, active: bool) -> User

// Después (con Builder)
User::builder().name("Ana").age(25).email("ana@e.com").build()?
```

- [ ] Builder con tipos genéricos (5 pts)
- [ ] Validación en `build()` con `Result<User, UserError>` (5 pts)
- [ ] Doctest que funciona con `cargo test` (5 pts)
- [ ] Clippy limpio (5 pts)

### Tipos de error con `thiserror` (20 pts)

- [ ] Enum de errores con variantes descriptivas (5 pts)
- [ ] `#[error(...)]` attributes en todas las variantes (5 pts)
- [ ] `#[from]` para conversiones automáticas (5 pts)
- [ ] Test de cada variante de error (5 pts)

---

## 🏗️ Evaluación de Producto (30 pts)

### Proyecto: `project-mi-crate` (30 pts)

Librería lista para publicación:

| Criterio | Puntos |
|----------|--------|
| `#![deny(missing_docs)]` — toda la API documentada | 8 pts |
| Doctests funcionales en todas las funciones públicas | 8 pts |
| Tipos de error con `thiserror` | 6 pts |
| `Cargo.toml` con metadata completa (description, license, keywords) | 4 pts |
| `cargo test && cargo clippy -- -D warnings` | 4 pts |

---

## 📈 Escala de Calificación

| Rango | Nota |
|-------|------|
| 90-100 | Sobresaliente |
| 75-89 | Notable |
| 60-74 | Aprobado |
| < 60 | Suspendido |
