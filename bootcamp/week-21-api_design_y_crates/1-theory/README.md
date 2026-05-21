# Semana 21 — Teoría: Diseño de APIs y `crates.io`

## Archivos de Teoría

| # | Archivo | Tema | Diagrama |
|---|---------|------|----------|
| 01 | [01-api-guidelines.md](01-api-guidelines.md) | Rust API Guidelines, naming, newtypes | [SVG](../0-assets/01-api-guidelines.svg) |
| 02 | [02-builder-typestate.md](02-builder-typestate.md) | Patrón Builder clásico y con typestate | [SVG](../0-assets/02-builder-typestate.svg) |
| 03 | [03-error-types.md](03-error-types.md) | Tipos de error custom con `thiserror` | [SVG](../0-assets/03-error-types.svg) |
| 04 | [04-docs-doctests.md](04-docs-doctests.md) | Documentación, doctests y `examples/` | [SVG](../0-assets/04-docs-doctests.svg) |
| 05 | [05-semver-publish.md](05-semver-publish.md) | Semver, CHANGELOG y `cargo publish` | [SVG](../0-assets/05-semver-publish.svg) |

---

## Mapa Conceptual

```
Diseño de APIs y crates.io
│
├── Rust API Guidelines
│   ├── Naming conventions (snake_case, PascalCase, SCREAMING)
│   ├── Tipos genéricos — impl Into<T>, impl AsRef<T>
│   ├── Newtype pattern — semántica sobre primitivos
│   ├── Métodos vs. funciones — cuándo usar cada uno
│   └── Consistencia: new, default, from, into, as_*
│
├── Patrón Builder
│   ├── Builder clásico — campos con Option<T>
│   ├── Builder typestate — estados en phantom types
│   ├── Fluent interface — métodos que retornan Self
│   └── Validación en build() → Result<T, E>
│
├── Manejo de Errores en APIs
│   ├── thiserror — derive Error para librerías
│   ├── anyhow — manejo en binarios y aplicaciones
│   ├── #[error("...")] — mensajes descriptivos
│   ├── #[from] — conversiones automáticas
│   └── Jerarquía de errores públicos vs. internos
│
├── Documentación y Doctests
│   ├── /// vs //! — item vs. módulo
│   ├── Secciones: Examples, Errors, Panics, Safety
│   ├── Doctests — código en docs que cargo test ejecuta
│   ├── #[deny(missing_docs)] — API pública documentada
│   └── examples/ — binarios de ejemplo independientes
│
└── Semver y Publicación
    ├── MAJOR.MINOR.PATCH — qué rompe la compatibilidad
    ├── CHANGELOG.md — formato keep-a-changelog
    ├── #[deprecated] — marcar API obsoleta
    ├── Metadata Cargo.toml: description, license, keywords
    └── cargo publish --dry-run → cargo publish
```

---

## Herramientas de la Semana

| Herramienta | Propósito | Instalación |
|-------------|-----------|-------------|
| `thiserror` | Derivar `Error` para tipos custom | `cargo add thiserror@2.0.12` |
| `cargo doc --open` | Generar y ver documentación HTML | incluido en Rust |
| `cargo test --doc` | Ejecutar solo los doctests | incluido en Rust |
| `cargo publish --dry-run` | Simular publicación | incluido en Cargo |
| `cargo package --list` | Ver archivos que se incluirán | incluido en Cargo |

---

## Progresión de Aprendizaje

```
Semana 18 (Macros)
    └── Semana 19 (unsafe)
            └── Semana 20 (FFI)
                    └── Semana 21 (API Design + crates.io)  ← aquí
                                │
                                └── Ahora construyes librerías públicas completas
```

La semana 21 es el punto de integración de las fases 6 y 7: combinas todo
lo aprendido para crear crates listos para `crates.io`, con APIs bien diseñadas,
documentadas, versionadas y publicables.
