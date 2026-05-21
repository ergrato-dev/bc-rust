---
applyTo: "bootcamp/**/2-practice/**"
---

# Estructura de Ejercicios — Bootcamp bc-rust

## Layout obligatorio de cada práctica

```
practice-XX-nombre-descriptivo/
├── Cargo.toml          — nombre del crate: practice-XX-nombre
├── src/
│   ├── main.rs         — punto de entrada (o lib.rs para librerías)
│   └── lib.rs          — si el ejercicio tiene lógica reutilizable
├── tests/
│   └── integration_test.rs   — tests de integración (cuando aplique)
└── README.md           — instrucciones del ejercicio
```

## Cargo.toml mínimo

```toml
[package]
name    = "practice-01-nombre"
version = "0.1.0"
edition = "2021"

# Solo agregar dependencias cuando el ejercicio las requiera explícitamente
```

## README.md de ejercicio (estructura obligatoria)

```markdown
# Práctica XX — Nombre del Ejercicio

## 🎯 Objetivo
Una línea clara describiendo qué aprenderá el estudiante.

## 📋 Instrucciones
Pasos numerados y concretos de lo que debe implementar.

## ✅ Criterios de Aceptación
- [ ] El programa compila sin warnings
- [ ] Pasa todos los tests con `cargo test`
- [ ] El código supera `cargo clippy -- -D warnings`
- [ ] [criterios específicos del ejercicio]

## 💡 Pistas
Hints opcionales que no revelan la solución.

## 🔗 Referencias
- [Sección del Rust Book relevante](https://doc.rust-lang.org/book/...)
```

## Scaffolding de código inicial

Los ejercicios deben incluir scaffolding con `todo!()` o comentarios guía:

```rust
// ✅ Correcto — el estudiante sabe dónde implementar
pub fn calculate_area(width: f64, height: f64) -> f64 {
    // TODO: calcular el área del rectángulo
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_positiva() {
        assert_eq!(calculate_area(3.0, 4.0), 12.0);
    }

    #[test]
    fn test_area_cero() {
        assert_eq!(calculate_area(0.0, 5.0), 0.0);
    }
}
```

## Proyectos integradores (carpeta `3-project/`)

Los proyectos semanales siguen la misma estructura pero con:
- `README.md` más extenso con contexto del dominio
- Múltiples módulos en `src/`
- Tests de integración obligatorios en `tests/`
- `#![deny(missing_docs)]` en el crate root

## Reglas de naming

| Elemento | Patrón | Ejemplo |
|----------|--------|---------|
| Carpeta práctica | `practice-NN-nombre` | `practice-01-hello-world` |
| Nombre crate | igual a carpeta | `practice-01-hello-world` |
| Carpeta proyecto | `project-nombre` | `project-inventory-system` |
| Nombre crate proyecto | `project-nombre` | `project-inventory-system` |
