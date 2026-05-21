---
applyTo: "**/*.rs"
---

# Reglas de Código Rust — Bootcamp bc-rust

## Convenciones de nombrado

- `snake_case` — funciones, variables, módulos, campos
- `PascalCase` — tipos, traits, enums, structs
- `SCREAMING_SNAKE_CASE` — constantes y estáticas
- Prefijo `_` — variables intencionalmente no usadas

## Estilo general

- Formatear siempre con `rustfmt` (no editar formato a mano)
- Máximo 100 caracteres por línea
- Bloques `use` agrupados: `std::` primero, luego crates externos, luego crate local
- No usar `use *` (glob imports) excepto en `#[cfg(test)] use super::*`

## Manejo de errores

```rust
// ✅ En código de producción / librerías
fn parse(s: &str) -> Result<u32, ParseError> { ... }

// ✅ En ejemplos didácticos (semanas 1-6) — aceptable con comentario
let n = s.parse::<u32>().unwrap(); // solo en ejemplos, no en código real

// ❌ Nunca en código de proyecto semanal
fn main() {
    let val = some_result.unwrap(); // prohibido en proyectos
}
```

## Documentación

```rust
/// Descripción de la función en una línea.
///
/// # Arguments
/// * `x` - descripción del parámetro
///
/// # Returns
/// Descripción del valor de retorno.
///
/// # Errors
/// Describe cuándo retorna `Err`.
///
/// # Examples
/// ```
/// let result = my_fn(42);
/// assert_eq!(result, Ok(84));
/// ```
pub fn my_fn(x: i32) -> Result<i32, MyError> { ... }
```

- Documentar **todas** las funciones y tipos `pub`
- Incluir `# Examples` con doctests ejecutables
- Usar `# Errors` cuando la función retorna `Result`
- Usar `# Panics` cuando la función puede entrar en pánico

## Unsafe (semanas 19+)

```rust
// SAFETY: el puntero `ptr` es válido porque fue obtenido de un Box::into_raw
// y no ha sido liberado aún. No hay otros accesos concurrentes activos.
unsafe {
    *ptr = 42;
}
```

- **Siempre** documentar el bloque `unsafe` con `// SAFETY:` inmediatamente antes
- Minimizar el alcance de bloques `unsafe`
- Encapsular `unsafe` dentro de abstracciones seguras cuando sea posible

## Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nombre_descriptivo() {
        // Arrange
        let input = 42;
        // Act
        let result = my_fn(input);
        // Assert
        assert_eq!(result, Ok(84));
    }
}
```

- Nombre de test: describe qué hace, no cómo
- Patrón Arrange-Act-Assert
- Un assert conceptual por test (pueden ser múltiples líneas)
- Tests de error también obligatorios: `#[should_panic]` o comprobar `Err`

## Clippy

El código del bootcamp debe pasar `cargo clippy -- -D warnings`. Excepciones documentadas:
```rust
#[allow(clippy::too_many_arguments)] // justificación aquí
```
