# 🦀 Semana 25: Capstone — "Rewrite It In Rust" (RIIR)

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 25 de 25 |
| **Tema** | Proyecto final integrador — librería real con bindings |
| **Duración** | 4 horas |
| **Nivel** | Avanzado — Proyecto final del bootcamp |
| **Requisitos** | Semanas 18–24 completadas |

---

## 🎯 Objetivo General

Construir una librería Rust de calidad de producción que:
- Resuelve un problema real
- Expone una API pública bien diseñada (`#![deny(missing_docs)]`)
- Tiene tests exhaustivos
- Integra con al menos un ecosistema externo (Python, Node.js, WASM, C)

---

## 🗺️ Opciones del Capstone

| Opción | Proyecto | Stack |
|--------|----------|-------|
| **A** | Parser/lexer expuesto a Python | Rust + PyO3 + maturin |
| **B** | CLI tool que reemplaza una herramienta | Rust + clap + indicatif |
| **C** | Motor numérico compilado a WASM | Rust + wasm-pack + TypeScript |
| **D** | Librería criptográfica con API C | Rust + cbindgen + unsafe |

---

## 📋 Criterios Comunes (todas las opciones)

- [ ] `#![deny(missing_docs)]` — toda la API pública documentada
- [ ] `cargo clippy -- -D warnings` pasa sin errores
- [ ] `cargo test` — cobertura de las funciones principales
- [ ] `cargo fmt --check` — código formateado
- [ ] `README.md` completo con ejemplos de uso
- [ ] Versión semver correcta en `Cargo.toml`
