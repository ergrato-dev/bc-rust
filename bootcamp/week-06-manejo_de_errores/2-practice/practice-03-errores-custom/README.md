# Práctica 03: Errores Personalizados

## 🎯 Objetivo

Crear tipos de error expresivos que proporcionen contexto útil para debugging.

## 📋 Ejercicios

### Ejercicio 1: Definir el Enum

Crea un enum `ConfigError` con variantes para diferentes tipos de errores:

```rust
enum ConfigError {
    ArchivoNoEncontrado(String),
    ErrorLectura(io::Error),
    CampoFaltante(String),
    ValorInvalido { campo, valor, mensaje },
}
```

### Ejercicio 2: Implementar Display

Mensajes legibles para cada variante:

```rust
impl fmt::Display for ConfigError { ... }
```

### Ejercicio 3: Implementar Error

Con `source()` para encadenar errores:

```rust
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
}
```

### Ejercicio 4: Implementar From

Para conversión automática con `?`:

```rust
impl From<io::Error> for ConfigError { ... }
```

### Ejercicio 5: Usar el Tipo

Función que usa el tipo de error:

```rust
fn cargar_config(ruta: &str) -> Result<Config, ConfigError>
```

## ▶️ Ejecución

```bash
cargo run
cargo test
```

## 💡 Pistas

- `#[derive(Debug)]` para Debug automático
- `write!(f, "mensaje")` para Display
- `Some(&self.causa)` para source()

## ✅ Criterios de Éxito

- [ ] Enum con todas las variantes
- [ ] Display muestra mensajes útiles
- [ ] source() retorna la causa cuando aplica
- [ ] From permite usar ? con io::Error
