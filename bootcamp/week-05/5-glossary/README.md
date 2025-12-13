# 📖 Glosario - Semana 05: Enums y Pattern Matching

## Términos Clave

### Enum (Enumeración)
Tipo de dato que puede ser una de varias **variantes** posibles. En Rust, cada variante puede contener datos diferentes.

```rust
enum Resultado {
    Exito(i32),
    Error(String),
}
```

### Variante
Cada uno de los valores posibles de un enum. Las variantes pueden ser:
- **Unit**: Sin datos (`Variante`)
- **Tuple**: Con datos posicionales (`Variante(T, U)`)
- **Struct**: Con campos nombrados (`Variante { campo: T }`)

### Pattern Matching
Mecanismo para comparar un valor contra patrones y ejecutar código según el patrón que coincida.

```rust
match valor {
    Patron1 => accion1,
    Patron2 => accion2,
}
```

### Exhaustividad
Propiedad de `match` que garantiza que todos los posibles valores sean manejados. El compilador verifica esto.

### Wildcard (`_`)
Patrón que coincide con cualquier valor y lo ignora.

```rust
match x {
    1 => "uno",
    _ => "otro",  // Todo lo demás
}
```

### Guard (Guarda)
Condición adicional en un brazo de match usando `if`.

```rust
match x {
    n if n > 0 => "positivo",
    _ => "no positivo",
}
```

### Binding (`@`)
Captura el valor mientras se verifica un patrón.

```rust
match edad {
    n @ 0..=17 => println!("Menor: {}", n),
    _ => println!("Adulto"),
}
```

### Option<T>
Enum estándar para valores que pueden o no existir.

```rust
enum Option<T> {
    Some(T),  // Hay valor
    None,     // No hay valor
}
```

### Result<T, E>
Enum estándar para operaciones que pueden fallar.

```rust
enum Result<T, E> {
    Ok(T),   // Éxito
    Err(E),  // Error
}
```

### if let
Sintaxis concisa para manejar un solo patrón.

```rust
if let Some(x) = opcional {
    usar(x);
}
```

### while let
Iterar mientras un patrón coincida.

```rust
while let Some(x) = iter.next() {
    procesar(x);
}
```

### let else
Extraer valor o salir de la función.

```rust
let Some(x) = opcional else {
    return;
};
```

### Operador `?`
Propaga errores automáticamente (early return).

```rust
let valor = operacion()?;  // Retorna Err si falla
```

### Destructuring
Extraer valores de estructuras compuestas.

```rust
let (x, y) = tupla;
let Punto { x, y } = punto;
```

### matches! Macro
Verificar si un valor coincide con un patrón.

```rust
if matches!(estado, Estado::Activo) {
    // ...
}
```

### Algebraic Data Types (ADT)
Nombre formal para enums con datos. Combinan "tipos suma" (OR) con "tipos producto" (AND/struct).

### Discriminante
Valor interno que identifica qué variante está activa en un enum.

### Null Safety
Rust evita null usando `Option<T>`. No hay valores nulos implícitos.

### Unwrap
Método que extrae el valor interno o causa pánico si es None/Err.

```rust
let valor = Some(42).unwrap();  // 42
let boom = None.unwrap();       // ¡Pánico!
```

---

## Patrones Disponibles

| Patrón | Ejemplo | Descripción |
|--------|---------|-------------|
| Literal | `42` | Valor exacto |
| Variable | `x` | Captura el valor |
| Wildcard | `_` | Ignora el valor |
| Referencia | `&x` | Coincide referencia |
| Rango | `1..=10` | Rango inclusivo |
| OR | `A \| B` | Múltiples opciones |
| Guard | `x if x > 0` | Con condición |
| Binding | `x @ 1..=10` | Captura + verifica |
| Tuple | `(a, b, _)` | Destructura tupla |
| Struct | `S { x, y }` | Destructura struct |
| Enum | `E::V(x)` | Destructura variante |
| Slice | `[a, b, ..]` | Destructura slice |
| Rest | `..` | Ignora resto |

---

## Métodos Comunes

### Option<T>

| Método | Descripción |
|--------|-------------|
| `is_some()` | ¿Tiene valor? |
| `is_none()` | ¿Está vacío? |
| `unwrap()` | Extrae o pánico |
| `unwrap_or(default)` | Extrae o default |
| `unwrap_or_else(f)` | Extrae o calcula |
| `map(f)` | Transforma Some |
| `and_then(f)` | Encadena Options |
| `ok_or(err)` | Option → Result |
| `as_ref()` | &Option<T> → Option<&T> |

### Result<T, E>

| Método | Descripción |
|--------|-------------|
| `is_ok()` | ¿Es éxito? |
| `is_err()` | ¿Es error? |
| `unwrap()` | Extrae Ok o pánico |
| `unwrap_err()` | Extrae Err o pánico |
| `ok()` | Result → Option (descarta Err) |
| `err()` | Result → Option (descarta Ok) |
| `map(f)` | Transforma Ok |
| `map_err(f)` | Transforma Err |
| `and_then(f)` | Encadena Results |
