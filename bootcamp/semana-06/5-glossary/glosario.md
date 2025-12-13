# 📖 Glosario - Semana 06: Manejo de Errores

## Términos Fundamentales

### Result<T, E>
Tipo enum que representa el resultado de una operación que puede fallar. Tiene dos variantes: `Ok(T)` para éxito y `Err(E)` para error.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### panic!
Macro que causa la terminación abrupta del programa (o del thread). Se usa para errores irrecuperables o bugs.

```rust
panic!("Algo salió terriblemente mal");
```

### unwrap()
Método que extrae el valor de un `Result` u `Option`. Hace panic si es `Err` o `None`.

```rust
let valor = resultado.unwrap(); // ¡Peligroso!
```

### expect(msg)
Similar a `unwrap()` pero permite especificar un mensaje de error personalizado.

```rust
let valor = resultado.expect("El archivo debería existir");
```

### Operador ?
Operador que propaga errores automáticamente. Extrae el valor si es `Ok`, retorna el error si es `Err`.

```rust
let valor = operacion()?; // Propaga Err si falla
```

---

## Métodos de Result

### map(f)
Transforma el valor `Ok(T)` a `Ok(U)` usando la función f. No afecta `Err`.

```rust
let doble = Ok(5).map(|x| x * 2); // Ok(10)
```

### map_err(f)
Transforma el valor `Err(E)` a `Err(F)` usando la función f. No afecta `Ok`.

```rust
let mejor_error = Err("error").map_err(|e| format!("Falló: {}", e));
```

### and_then(f)
Encadena operaciones que retornan Result. También conocido como `flatMap` o `bind`.

```rust
let resultado = Ok(5).and_then(|x| dividir(x, 2));
```

### or_else(f)
Maneja el error y retorna otro Result. Útil para recuperación.

```rust
let valor = leer_cache().or_else(|_| leer_archivo());
```

### unwrap_or(default)
Retorna el valor `Ok` o el default proporcionado.

```rust
let puerto = config.parse().unwrap_or(8080);
```

### unwrap_or_else(f)
Como `unwrap_or` pero el default se calcula con un closure (lazy).

```rust
let config = leer_config().unwrap_or_else(|_| Config::default());
```

---

## Trait Error

### std::error::Error
Trait que define el comportamiento de un tipo de error.

```rust
trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}
```

### source()
Método que retorna la causa subyacente de un error, si existe.

```rust
impl Error for MiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.causa)
    }
}
```

### Display
Trait para mostrar un error de forma legible para usuarios.

### Debug
Trait para mostrar un error con información de debugging.

---

## Trait From

### From<T>
Trait para conversión de tipos. Permite que `?` convierta errores automáticamente.

```rust
impl From<io::Error> for MiError {
    fn from(err: io::Error) -> Self {
        MiError::Io(err)
    }
}
```

### Into<T>
El inverso de `From`. Se implementa automáticamente cuando implementas `From`.

---

## Tipos de Error Comunes

### std::io::Error
Error de operaciones de entrada/salida (archivos, red, etc.).

### std::num::ParseIntError
Error al parsear un string a número entero.

### std::num::ParseFloatError
Error al parsear un string a número flotante.

### Box<dyn Error>
Tipo que puede contener cualquier error. Útil para aplicaciones.

```rust
fn main() -> Result<(), Box<dyn Error>> {
    // ...
}
```

---

## Patrones

### Recoverable Error
Error del cual el programa puede recuperarse. Se maneja con `Result`.

### Unrecoverable Error
Error que indica un bug o estado inválido. Se maneja con `panic!`.

### Error Propagation
Técnica de pasar errores hacia arriba en la pila de llamadas usando `?`.

### Error Context
Información adicional agregada a un error para facilitar debugging.

### Type Alias para Result
Patrón común en bibliotecas para simplificar firmas.

```rust
pub type Result<T> = std::result::Result<T, MiError>;
```

---

## Crates Populares

### thiserror
Macro derive para implementar Error fácilmente. Ideal para bibliotecas.

```rust
#[derive(Error, Debug)]
enum MiError {
    #[error("Error de I/O: {0}")]
    Io(#[from] io::Error),
}
```

### anyhow
Manejo de errores simplificado para aplicaciones. Proporciona contexto.

```rust
use anyhow::{Context, Result};

fn foo() -> Result<()> {
    operacion().context("Falló la operación")?;
    Ok(())
}
```

### eyre
Alternativa a anyhow con mejor soporte para reportes de error.
