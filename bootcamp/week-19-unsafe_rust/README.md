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

| Archivo | Tema |
|---------|------|
| [1-theory/README.md](1-theory/README.md) | Teoría completa de la semana |
| [2-practice/](2-practice/) | Prácticas y proyecto |
| [4-resources/RECURSOS.md](4-resources/RECURSOS.md) | Referencias y lecturas |
| [5-glossary/README.md](5-glossary/README.md) | Glosario de términos |

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
cargo test -p project-unsafe-collections
```

---

## ⚠️ Regla de Oro

Todo bloque `unsafe` **debe** ir precedido de un comentario `// SAFETY:` que explique por qué el código es correcto. Sin justificación → el código no pasa revisión.

```rust
// SAFETY: `ptr` fue creado con `Box::into_raw` en esta función.
// Tenemos ownership exclusivo; nadie más tiene acceso a este puntero.
unsafe { *ptr = 42; }
```
