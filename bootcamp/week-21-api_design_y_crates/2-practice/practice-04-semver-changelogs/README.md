# Práctica 04 — Semver y CHANGELOGs

## 🎯 Objetivo
Comprender semver y practicar la evolución controlada de una API pública.

## 📋 Instrucciones

1. Partir de la API v0.1.0 de `Calculadora`
2. Agregar funcionalidades (minor bump v0.1.0 → v0.2.0):
   - `Calculadora::con_precision(usize)` — nuevo constructor
   - `Calculadora::historial()` — nuevo método
3. Hacer un cambio rompedor (major bump → v1.0.0):
   - Cambiar `dividir(a, b)` para retornar `Result<f64, _>`
4. Escribir un `CHANGELOG.md` con formato Keep-a-Changelog
5. Marcar funciones deprecadas con `#[deprecated(since = "0.2.0", note = "...")]`

## ✅ Criterios de Aceptación

- [ ] `Cargo.toml` tiene la versión correcta (`version = "0.2.0"`)
- [ ] CHANGELOG.md con secciones Added/Changed/Removed
- [ ] `#[deprecated]` usado en al menos 1 función
- [ ] `cargo test` pasa
- [ ] `cargo clippy -- -D warnings` pasa limpio (puede necesitar `#[allow(deprecated)]` en tests)
