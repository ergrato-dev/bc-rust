# 📊 Rúbrica de Evaluación — Semana 19: `unsafe` Rust

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **C1** | Usar raw pointers con verificación de null y gestión de lifetime |
| **C2** | Escribir funciones `unsafe` con invariantes documentados (`// SAFETY:`) |
| **C3** | Implementar `unsafe impl Send/Sync` con justificación semántica |
| **C4** | Aplicar `std::mem::transmute` y `size_of`/`align_of` correctamente |
| **C5** | Mantener el código `unsafe` mínimo y acotado |
| **C6** | Pasar `cargo clippy -- -D warnings` sin suprimir warnings unsafe |

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

1. ¿Cuáles son las 5 operaciones que solo se pueden realizar en un bloque `unsafe`? (5 pts)
2. ¿Cuál es la diferencia entre un *invariante de seguridad* y un *invariante de corrección*? (5 pts)
3. ¿Por qué `transmute` es la función más peligrosa de Rust? Dar un ejemplo de UB. (5 pts)

### Cuestionario Práctico (15 pts)

4. ¿Qué ocurre si se desreferencia un raw pointer nulo? (3 pts)
5. ¿Qué significa que un tipo implemente `Send`? ¿Y `Sync`? (4 pts)
6. ¿Cuándo es correcto usar `mem::forget`? (4 pts)
7. ¿Qué diferencia hay entre `*const T` y `*mut T`? (4 pts)

---

## ⚙️ Evaluación de Desempeño (40 pts)

### Ejercicio en Clase (20 pts)

Implementar un wrapper seguro sobre un raw pointer:

```rust
pub struct SafePtr<T> {
    ptr: *mut T,
}

impl<T> SafePtr<T> {
    pub fn new(value: T) -> Self { ... }
    pub fn get(&self) -> &T { ... }
    pub fn get_mut(&mut self) -> &mut T { ... }
}

impl<T> Drop for SafePtr<T> {
    fn drop(&mut self) { ... }
}
```

**Criterios**:
- [ ] Cada bloque `unsafe` tiene `// SAFETY:` (5 pts)
- [ ] `Drop` libera la memoria correctamente (5 pts)
- [ ] No hay memory leaks con Miri (5 pts)
- [ ] Tests que verifican comportamiento (5 pts)

### Debugging en Tiempo Real (20 pts)

Depurar código `unsafe` con UB conocido y explicar la causa:
- [ ] Identificar el tipo de UB (use-after-free, double-free, etc.) (10 pts)
- [ ] Proponer y aplicar la corrección (10 pts)

---

## 🏗️ Evaluación de Producto (30 pts)

### Proyecto: `project-unsafe-collections` (30 pts)

Implementar una estructura de datos `RawVec<T>` mínima que gestione memoria manualmente:

| Criterio | Puntos |
|----------|--------|
| `push`, `pop`, `len`, `capacity` funcionales | 10 pts |
| Gestión correcta de crecimiento (realloc) con `// SAFETY:` | 8 pts |
| `Drop` que libera la memoria sin double-free | 5 pts |
| Tests con `cargo test` (mínimo 8 tests) | 5 pts |
| Clippy sin warnings | 2 pts |

---

## 📈 Escala de Calificación

| Rango | Nota | Descripción |
|-------|------|-------------|
| 90-100 | Sobresaliente | Dominio completo, código production-quality |
| 75-89 | Notable | Comprensión sólida con errores menores |
| 60-74 | Aprobado | Conceptos básicos correctos |
| < 60 | Suspendido | Necesita refuerzo de fundamentos |
