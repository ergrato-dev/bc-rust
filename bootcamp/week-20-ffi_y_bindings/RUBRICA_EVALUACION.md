# 📊 Rúbrica de Evaluación — Semana 20: FFI y Language Bindings

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **C1** | Consumir funciones C desde Rust con `extern "C"` |
| **C2** | Exportar funciones Rust a C con `#[no_mangle]` y `#[repr(C)]` |
| **C3** | Crear una extensión Python básica con PyO3 |
| **C4** | Gestionar strings y ownership en la frontera FFI |
| **C5** | Documentar invariantes de seguridad en código FFI |
| **C6** | Usar `CStr`/`CString` correctamente para strings interop |

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

1. ¿Qué hace `#[repr(C)]` y por qué es obligatorio en FFI? (5 pts)
2. ¿Cuál es la diferencia entre `CStr` y `CString`? (5 pts)
3. ¿Qué es el ABI y por qué importa en `extern "C"`? (5 pts)
4. ¿Cómo se gestiona la memoria cuando Rust crea un objeto y lo pasa a C? (5 pts)
5. ¿Qué hace `#[no_mangle]`? ¿Qué problema resuelve? (5 pts)
6. ¿Por qué los tipos `bool` de Rust y C no son interoperables directamente? (5 pts)

---

## ⚙️ Evaluación de Desempeño (40 pts)

### Ejercicio: Envolver librería C (20 pts)

Crear un wrapper seguro sobre `libc::strlen`:

```rust
pub fn strlen_safe(s: &str) -> usize { ... }
```

- [ ] Usa `CString::new` correctamente (5 pts)
- [ ] Tiene `// SAFETY:` en el bloque unsafe (5 pts)
- [ ] Tests que verifican strings con y sin bytes nulos (10 pts)

### Ejercicio: Exportar a C (20 pts)

Crear una función `add_vectors` que C pueda llamar:

- [ ] `#[repr(C)]` en el struct del resultado (5 pts)
- [ ] `#[no_mangle] pub extern "C" fn` (5 pts)
- [ ] Manejo de puntero null (5 pts)
- [ ] Header C generado con `cbindgen` o escrito manualmente (5 pts)

---

## 🏗️ Evaluación de Producto (30 pts)

### Proyecto: `project-libreria-ffi` (30 pts)

Implementar una librería Rust que exponga una API C completa:

| Criterio | Puntos |
|----------|--------|
| Structs con `#[repr(C)]` correctos | 6 pts |
| Funciones `create`/`destroy`/`use` con ownership claro | 8 pts |
| Manejo de errores via código de retorno (no panic) | 6 pts |
| `// SAFETY:` en todos los bloques unsafe | 5 pts |
| Tests de integración que verifican el ciclo completo | 5 pts |

---

## 📈 Escala de Calificación

| Rango | Nota |
|-------|------|
| 90-100 | Sobresaliente |
| 75-89 | Notable |
| 60-74 | Aprobado |
| < 60 | Suspendido |
