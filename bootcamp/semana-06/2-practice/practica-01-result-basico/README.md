# Práctica 01: Result Básico

## 🎯 Objetivo

Aprender a usar `Result<T, E>` para manejar errores de forma segura y expresiva.

## 📋 Ejercicios

### Ejercicio 1: División Segura

Implementa una función que divide dos números, retornando error si el divisor es cero.

```rust
fn dividir(dividendo: f64, divisor: f64) -> Result<f64, String>
```

### Ejercicio 2: Parsear Edad

Parsea un string a edad (u8), validando:
- Que sea un número válido
- Que no sea negativo
- Que esté en rango 0-150

### Ejercicio 3: Encadenar Operaciones

Combina parseo y división usando `and_then` o el operador `?`.

### Ejercicio 4: Valor por Defecto

Usa `unwrap_or` para proporcionar un valor default cuando el parseo falla.

## ▶️ Ejecución

```bash
# Ejecutar
cargo run

# Verificar con tests
cargo test
```

## ✅ Criterios de Éxito

- [ ] Todas las funciones implementadas
- [ ] Tests pasan (`cargo test`)
- [ ] Sin warnings (`cargo clippy`)

## 💡 Pistas

- `parse::<T>()` retorna `Result<T, ParseError>`
- `map_err(|_| "mensaje")` convierte el tipo de error
- `and_then(|x| otra_operacion(x))` encadena Results
- `unwrap_or(default)` retorna el valor o un default
