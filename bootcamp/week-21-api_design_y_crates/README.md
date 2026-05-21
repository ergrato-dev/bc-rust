# 🦀 Semana 21: Diseño de APIs y Publicación en `crates.io`

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 21 de 25 |
| **Tema** | Ergonomía de APIs Rust, semver, doctests, publicación en crates.io |
| **Duración** | 4 horas |
| **Nivel** | Avanzado — Fase de Diseño de Librerías |
| **Requisitos** | Semanas 18-20, sólido dominio de traits y generics |

---

## 🎯 Objetivos de Aprendizaje

1. **Diseñar** APIs ergonómicas siguiendo las Rust API Guidelines
2. **Implementar** el patrón Builder para APIs complejas
3. **Escribir** doctests completos y ejemplos en `examples/`
4. **Gestionar** errores con tipos custom usando `thiserror`
5. **Aplicar** semver y gestionar el `CHANGELOG`
6. **Preparar** y publicar un crate en `crates.io`

---

## 📚 Contenido

| Archivo | Tema |
|---------|------|
| [1-theory/README.md](1-theory/README.md) | Teoría completa de la semana |
| [2-practice/](2-practice/) | Prácticas y proyecto |
| [4-resources/RECURSOS.md](4-resources/RECURSOS.md) | Referencias y lecturas |
| [5-glossary/README.md](5-glossary/README.md) | Glosario de términos |

---

## 🔑 Principios de las Rust API Guidelines

- **Predictibilidad**: nombres y comportamientos consistentes
- **Flexibilidad**: tomar tipos genéricos donde sea razonable (`impl Into<String>`)
- **Tipos explícitos**: preferir tipos newtype sobre primitivos
- **Errores tipados**: `Result<T, MyError>` con `thiserror`, no `Box<dyn Error>`
- **Sin `unwrap`**: en APIs públicas, siempre propagar errores
