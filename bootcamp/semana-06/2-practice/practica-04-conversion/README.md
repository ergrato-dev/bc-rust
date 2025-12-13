# Práctica 04: Conversión de Errores

## 🎯 Objetivo

Dominar técnicas de conversión entre tipos de error y agregar contexto.

## 📋 Ejercicios

### Ejercicio 1: map_err

Convierte errores a mensajes descriptivos:

```rust
fn parsear_puerto(s: &str) -> Result<u16, String>
```

### Ejercicio 2: Box<dyn Error>

Maneja errores heterogéneos:

```rust
fn leer_y_parsear(ruta: &str) -> Result<i32, Box<dyn Error>>
```

### Ejercicio 3: Agregar Contexto

Envuelve errores con información adicional:

```rust
fn leer_con_contexto(ruta: &str) -> Result<i32, ErrorConContexto>
```

### Ejercicio 4: Múltiples Operaciones

Procesa varios archivos con manejo de errores:

```rust
fn sumar_archivos(rutas: &[&str]) -> Result<i64, String>
```

### Bonus: Collect Results

```rust
fn parsear_todos(strings: &[&str]) -> Result<Vec<i32>, String>
```

## ▶️ Ejecución

```bash
cargo run
cargo test
```

## 💡 Pistas

```rust
// map_err convierte el tipo de error
.map_err(|e| format!("Error: {}", e))

// Box<dyn Error> acepta cualquier error
fn foo() -> Result<T, Box<dyn Error>> {
    let x = operacion_io()?;      // io::Error → Box<dyn Error>
    let y = operacion_parse()?;   // ParseError → Box<dyn Error>
    Ok(...)
}

// collect() puede recolectar Results
let nums: Result<Vec<i32>, _> = strings.iter()
    .map(|s| s.parse())
    .collect();
```

## ✅ Criterios de Éxito

- [ ] Conversiones de error correctas
- [ ] Mensajes con contexto útil
- [ ] Tests pasan
