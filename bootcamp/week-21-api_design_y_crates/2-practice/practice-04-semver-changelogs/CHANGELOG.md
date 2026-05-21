# Changelog

Todos los cambios notables en este proyecto seguirán el formato [Keep a Changelog](https://keepachangelog.com/es/1.0.0/).

Este proyecto respeta [Semantic Versioning](https://semver.org/lang/es/).

---

## [Unreleased]

---

## [0.2.0] — 2026-05-21

### Added

- `con_precision(decimales: usize)` — configura los decimales de resultado.
- `historial() -> &[f64]` — devuelve el registro de operaciones previas.

### Changed

- `dividir(a, b)` ahora retorna `Result<f64, CalculadoraError>` en lugar de `f64`.
  - **Breaking change** respecto a 0.1.x: actualizar los callers para manejar el `Result`.

### Fixed

- Corregido panic no controlado al dividir por cero; ahora retorna `Err(CalculadoraError::DivisionPorCero)`.

---

## [0.1.0] — 2026-04-15

### Added

- `sumar(a: f64, b: f64) -> f64` — suma de dos números.
- `restar(a: f64, b: f64) -> f64` — resta de dos números.
- `multiplicar(a: f64, b: f64) -> f64` — multiplicación de dos números.
- `dividir(a: f64, b: f64) -> f64` — división (panic si b == 0.0).

---

[Unreleased]: https://github.com/example/calculadora/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/example/calculadora/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/example/calculadora/releases/tag/v0.1.0
