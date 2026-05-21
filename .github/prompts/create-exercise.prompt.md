---
mode: agent
description: "Genera un ejercicio Cargo completo con scaffolding, tests y README para el bootcamp."
---

Crea un ejercicio completo para la **semana ${input:weekNumber}** del bootcamp bc-rust.

**Nombre del ejercicio**: `practice-${input:exerciseNumber}-${input:exerciseSlug}`  
**Concepto a practicar**: ${input:concept}  
**Nivel de dificultad**: ${input:difficulty} (básico / intermedio / avanzado)

## Archivos a generar

### `Cargo.toml`
```toml
[package]
name    = "practice-${exerciseNumber}-${exerciseSlug}"
version = "0.1.0"
edition = "2021"
```
Solo agregar `[dependencies]` si el ejercicio genuinamente las necesita.

### `src/main.rs` (o `src/lib.rs`)

Estructura requerida:
1. Comentario de cabecera explicando qué se practica
2. Imports necesarios (solo los que ya puede usar el estudiante)
3. Funciones scaffolded con `todo!()` y comentarios guía
4. Módulo `#[cfg(test)]` con **mínimo 5 tests**:
   - Tests de caso normal (happy path)
   - Tests de caso borde (valores límite, vacío, negativo)
   - Tests de error (si aplica `Result`/`Option`)

Reglas de scaffolding:
- La firma de cada función debe estar completa (tipos correctos)
- Los tests deben estar escritos y compilar (aunque fallen con `todo!()`)
- Los comentarios guía no deben revelar la solución
- Marcar partes opcionales/bonus con `// BONUS:`

### `README.md`

Usar esta estructura exacta:

```markdown
# Práctica ${exerciseNumber} — Nombre Descriptivo

## 🎯 Objetivo
[Una oración. Qué aprende el estudiante al completar este ejercicio.]

## 📋 Instrucciones
[Pasos numerados: qué funciones implementar y qué debe hacer cada una.]

## ✅ Criterios de Aceptación
- [ ] Compila sin warnings: `cargo clippy -- -D warnings`
- [ ] Todos los tests pasan: `cargo test`
- [ ] [criterio específico 1]
- [ ] [criterio específico 2]

## 💡 Pistas
<details>
<summary>Pista 1 (expandir si estás atascado)</summary>

[Hint que orienta sin revelar la solución]

</details>

## 🔗 Referencias
- [Sección del Rust Book](https://doc.rust-lang.org/book/ch...)
```

## Consideraciones por nivel de dificultad

**Básico**: funciones puras, tipos primitivos, sin genéricos  
**Intermedio**: usa `Option`/`Result`, iteradores, traits estándar  
**Avanzado**: lifetimes explícitos, genéricos con bounds, traits custom  

## Ejemplo de output esperado

Al ejecutar `cargo test`, los tests deben fallar con `not yet implemented`
hasta que el estudiante complete las funciones. No usar `#[ignore]`.
