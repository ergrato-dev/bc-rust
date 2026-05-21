# 🦀 Semana 19: `unsafe` Rust y Raw Pointers

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 19 de 25 |
| **Tema** | `unsafe` Rust: raw pointers, bloques unsafe, FFI prep |
| **Duración** | 4 horas |
| **Nivel** | Avanzado — Fase de Diseño de Librerías |
| **Requisitos** | Semana 18 (Macros), sólido dominio de ownership y lifetimes |

---

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. **Comprender** qué operaciones requieren `unsafe` y por qué existe la distinción
2. **Usar** raw pointers (`*const T`, `*mut T`) de forma correcta y documentada
3. **Escribir** funciones `unsafe` con invariantes claros y documentados (`// SAFETY:`)
4. **Implementar** traits `unsafe` (`Send`, `Sync`) cuando sea semánticamente correcto
5. **Manipular** memoria con `std::mem` (`transmute`, `size_of`, `align_of`, `forget`)
6. **Aplicar** el checklist de revisión de código `unsafe` antes de cada merge

---

## 📚 Contenido

| Archivo | Tema | Diagrama |
|---------|------|----------|
| [1-theory/01-intro-unsafe.md](1-theory/01-intro-unsafe.md) | Los 5 superpoderes, UB, SAFETY comment | [SVG](0-assets/01-intro-unsafe.svg) |
| [1-theory/02-raw-pointers.md](1-theory/02-raw-pointers.md) | `*const T`, `*mut T`, aritmética, `NonNull<T>` | [SVG](0-assets/02-raw-pointers.svg) |
| [1-theory/03-unsafe-funciones.md](1-theory/03-unsafe-funciones.md) | `unsafe fn`, safe abstractions, `# Safety` | [SVG](0-assets/03-unsafe-funciones.svg) |
| [1-theory/04-unsafe-traits.md](1-theory/04-unsafe-traits.md) | `Send`, `Sync`, `unsafe impl`, `PhantomData` | [SVG](0-assets/04-unsafe-traits.svg) |
| [1-theory/05-std-mem.md](1-theory/05-std-mem.md) | `transmute`, `size_of`, `forget`, `MaybeUninit` | [SVG](0-assets/05-std-mem.svg) |
| [2-practice/](2-practice/) | Prácticas y proyecto | — |
| [4-resources/RECURSOS.md](4-resources/RECURSOS.md) | Referencias y lecturas | — |
| [5-glossary/README.md](5-glossary/README.md) | Glosario de términos | — |

### Prácticas

| # | Práctica | Descripción | Tiempo |
|---|----------|-------------|--------|
| 1 | [practice-01-raw-pointers](2-practice/practice-01-raw-pointers/) | Crear, verificar y desreferenciar raw pointers | 30 min |
| 2 | [practice-02-unsafe-funciones](2-practice/practice-02-unsafe-funciones/) | `unsafe fn` con precondiciones documentadas | 30 min |
| 3 | [practice-03-unsafe-traits](2-practice/practice-03-unsafe-traits/) | `unsafe impl Send/Sync` para wrapper C | 30 min |
| 4 | [practice-04-mem-y-transmute](2-practice/practice-04-mem-y-transmute/) | `std::mem`: `transmute`, `size_of`, `forget` | 30 min |
| P | [project-unsafe-collections](2-practice/project-unsafe-collections/) | `RawVec<T>`: Vec con gestión manual de memoria | 90 min |

---

## 🚀 Cómo Ejecutar

```bash
# Compilar todos los ejercicios de la semana
cargo build -p practice-01-raw-pointers
cargo build -p practice-02-unsafe-funciones
cargo build -p practice-03-unsafe-traits
cargo build -p practice-04-mem-y-transmute
cargo build -p project-unsafe-collections

# Ejecutar tests
cargo test -p practice-01-raw-pointers
cargo test -p practice-02-unsafe-funciones
cargo test -p practice-03-unsafe-traits
cargo test -p practice-04-mem-y-transmute
cargo test -p project-unsafe-collections

# Detectar UB con Miri (requiere nightly)
cargo +nightly miri test -p project-unsafe-collections
```

---

## ⚠️ Regla de Oro

Todo bloque `unsafe` **debe** ir precedido de un comentario `// SAFETY:` que explique por qué el código es correcto. Sin justificación → el código no pasa revisión.

```rust
// SAFETY: `ptr` fue creado con `Box::into_raw` en esta función.
// Tenemos ownership exclusivo; nadie más tiene acceso a este puntero.
unsafe { *ptr = 42; }
```
