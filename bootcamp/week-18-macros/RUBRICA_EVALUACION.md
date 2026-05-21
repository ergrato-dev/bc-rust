# 📊 Rúbrica de Evaluación — Semana 18: Macros

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **C1** | Crear macros declarativas con `macro_rules!` |
| **C2** | Aplicar patrones de matching y repetición en macros |
| **C3** | Configurar un workspace con crate proc-macro separado |
| **C4** | Implementar un `#[derive]` macro usando `syn` + `quote` |
| **C5** | Crear una macro de atributo que transforme código |
| **C6** | Diseñar macros seguras, documentadas y testeadas |

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
| 1 | ¿Cuál es la diferencia entre `macro_rules!` y proc-macros? | 3 |
| 2 | ¿Por qué las proc-macros deben estar en un crate separado? | 3 |
| 3 | ¿Qué hace el designator `:expr` en `macro_rules!`? | 3 |
| 4 | ¿Para qué sirve `syn::parse_macro_input!`? | 3 |
| 5 | ¿Qué es el `TokenStream` y cómo fluye por una proc-macro? | 3 |

### Código Conceptual (15 pts)

#### Pregunta 1 (5 pts): Identificar el error

```rust
macro_rules! mi_vec {
    [$($x:expr),+] => {
        {
            let mut v = Vec::new();
            $(v.push($x))+    // ← ¿qué falta aquí?
            v
        }
    };
}
```

**Respuesta esperada**: Falta el punto y coma en la repetición: `$(v.push($x);)*` — el `+` requiere al menos 1 elemento y falta `;` para separar los statements.

#### Pregunta 2 (5 pts): Completar la firma

```rust
// ¿Cuál es la firma correcta para una macro #[derive]?
pub fn mi_derive(/* ??? */) -> /* ??? */ {
    // ...
}
```

**Respuesta esperada**: `pub fn mi_derive(input: TokenStream) -> TokenStream` con `#[proc_macro_derive(MiDerive)]` sobre la función.

#### Pregunta 3 (5 pts): Describir la expansión

```rust
#[derive(Debug, Clone)]
struct Punto {
    x: f64,
    y: f64,
}
```

**Respuesta esperada**: El compilador invoca la proc-macro de `Debug` que genera un bloque `impl Debug for Punto { fn fmt(...) }` y otro `impl Clone for Punto { fn clone(...) }`. El código fuente no es modificado, solo se añaden implementaciones.

---

## 💻 Evaluación de Desempeño (40 pts)

### Práctica 1: macro_rules! Básico (8 pts)

| Criterio | Pts |
|----------|-----|
| `map!` macro funciona con 0, 1 y N pares clave-valor | 3 |
| `assert_matches!` macro con mensaje personalizado | 3 |
| Tests pasan con `cargo test` | 2 |

### Práctica 2: macro_rules! Avanzado (8 pts)

| Criterio | Pts |
|----------|-----|
| Macro recursiva implementada correctamente | 3 |
| Patrones con repetición `$()*`, `$()+`, `$(...)?` | 3 |
| Hygiene verificada con tests | 2 |

### Práctica 3: Custom Derive (12 pts)

| Criterio | Pts |
|----------|-----|
| Workspace configurado con crate derive separado | 3 |
| `syn::parse_macro_input!` usado correctamente | 3 |
| `quote::quote!` genera código válido | 3 |
| `#[derive(Describe)]` funciona en structs y enums | 3 |

### Práctica 4: Attribute Macro (12 pts)

| Criterio | Pts |
|----------|-----|
| Attribute macro implementada en crate separado | 3 |
| Parsing de argumentos del atributo | 3 |
| Transformación del item de código correcta | 3 |
| Tests de integración presentes | 3 |

---

## 📦 Evaluación de Producto (30 pts)

### Proyecto: macro-toolkit (30 pts)

| Criterio | Descripción | Pts |
|----------|-------------|-----|
| **Funcionalidad** | Macros funcionan según spec | 8 |
| **Correctitud** | `cargo test` pasa sin fallos | 6 |
| **Calidad** | `cargo clippy -- -D warnings` sin warnings | 4 |
| **Documentación** | Todas las macros públicas documentadas con `///` y ejemplos | 4 |
| **SAFETY** | Si hay `unsafe`, tiene comentario `// SAFETY:` | 3 |
| **Estructura** | Workspace con crates separados correctamente configurado | 3 |
| **Versiones** | Dependencias con versión exacta en todos los `Cargo.toml` | 2 |

---

## 📏 Escala de Calificación

| Puntos | Nivel | Descripción |
|--------|-------|-------------|
| 90-100 | ⭐ Excelente | Dominio completo del tema |
| 75-89 | ✅ Satisfactorio | Comprensión sólida, errores menores |
| 60-74 | ⚠️ En Progreso | Conceptos base, necesita práctica |
| < 60 | ❌ Insuficiente | Requiere reforzar fundamentos |

---

## ✅ Checklist de Entrega

Antes de entregar, verificar:

- [ ] `cargo build --workspace` compila sin errores
- [ ] `cargo test --workspace` pasa todos los tests
- [ ] `cargo clippy -- -D warnings` sin warnings
- [ ] `cargo fmt --check` no reporta diferencias
- [ ] Todas las macros públicas tienen documentación con ejemplos
- [ ] Dependencias con versión exacta (sin `^`, `~`, `*`, `>=`)
- [ ] `cargo audit --deny warnings` sin vulnerabilidades
- [ ] `cargo expand -p project-macro-toolkit` muestra expansión válida
