# ⚡ Option y Result

> **Manejo de ausencia y errores sin null ni excepciones**

![Option y Result](../0-assets/05-option-result.svg)

---

## El Problema de null

En otros lenguajes:

```javascript
// JavaScript - puede explotar en runtime
function obtenerUsuario(id) {
    let usuario = baseDatos.buscar(id);
    return usuario.nombre;  // 💥 null.nombre
}
```

Rust **no tiene null**. En su lugar: `Option<T>`.

---

## Option<T>

```rust
enum Option<T> {
    Some(T),  // Hay un valor
    None,     // No hay valor
}
```

Ejemplo:

```rust
fn buscar_usuario(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Ana"))
    } else {
        None
    }
}

fn main() {
    let usuario = buscar_usuario(1);
    
    // Debes manejar ambos casos
    match usuario {
        Some(nombre) => println!("Hola, {}", nombre),
        None => println!("Usuario no encontrado"),
    }
}
```

---

## Métodos de Option

### unwrap y expect

```rust
let valor = Some(42);

// unwrap: pánico si es None
let n = valor.unwrap();  // 42

// expect: pánico con mensaje personalizado
let n = valor.expect("Debería tener valor");
```

⚠️ **Evitar en producción** - solo para prototipos o cuando estás 100% seguro.

---

### unwrap_or

Valor por defecto:

```rust
let nombre = None;
let n = nombre.unwrap_or("Anónimo");  // "Anónimo"

let edad: Option<u32> = None;
let e = edad.unwrap_or(0);  // 0
```

---

### unwrap_or_else

Valor calculado solo si es necesario:

```rust
let config = None;
let c = config.unwrap_or_else(|| {
    cargar_config_por_defecto()  // Solo se ejecuta si es None
});
```

---

### map

Transformar el valor interno:

```rust
let numero: Option<i32> = Some(5);
let doble: Option<i32> = numero.map(|n| n * 2);  // Some(10)

let vacio: Option<i32> = None;
let doble: Option<i32> = vacio.map(|n| n * 2);  // None
```

---

### and_then (flatMap)

Para funciones que retornan Option:

```rust
fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

let resultado = Some(10)
    .and_then(|n| dividir(n, 2))   // Some(5)
    .and_then(|n| dividir(n, 0));  // None

// Sin and_then sería:
// Some(Some(Some(5))) - anidado
```

---

### is_some e is_none

```rust
let opt = Some(42);

if opt.is_some() {
    println!("Tiene valor");
}

if opt.is_none() {
    println!("Está vacío");
}
```

---

## Result<T, E>

Para operaciones que pueden **fallar**:

```rust
enum Result<T, E> {
    Ok(T),   // Éxito con valor T
    Err(E),  // Error de tipo E
}
```

---

## Ejemplo con Result

```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("División por cero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match dividir(10.0, 2.0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(error) => println!("Error: {}", error),
    }
}
```

---

## Métodos de Result

### unwrap y expect

Similar a Option:

```rust
let resultado: Result<i32, &str> = Ok(42);
let n = resultado.unwrap();  // 42

let error: Result<i32, &str> = Err("falló");
let n = error.unwrap();  // 💥 pánico
```

---

### unwrap_or y unwrap_or_else

```rust
let resultado: Result<i32, &str> = Err("error");

let n = resultado.unwrap_or(0);  // 0
let n = resultado.unwrap_or_else(|_| calcular_default());
```

---

### map y map_err

```rust
let resultado: Result<i32, &str> = Ok(5);

// Transformar el valor Ok
let doble = resultado.map(|n| n * 2);  // Ok(10)

// Transformar el error
let con_contexto = resultado.map_err(|e| format!("Error: {}", e));
```

---

### is_ok e is_err

```rust
let r: Result<i32, &str> = Ok(42);

if r.is_ok() {
    println!("Éxito!");
}

if r.is_err() {
    println!("Falló!");
}
```

---

## El Operador ?

**Propagación automática de errores**:

```rust
use std::fs::File;
use std::io::{self, Read};

fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    let mut archivo = File::open(ruta)?;  // Retorna Err si falla
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?;  // Retorna Err si falla
    Ok(contenido)
}
```

Es equivalente a:

```rust
fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    let mut archivo = match File::open(ruta) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    // ... resto igual
}
```

---

## ? con Option

También funciona con Option (en funciones que retornan Option):

```rust
fn obtener_primer_caracter(s: &str) -> Option<char> {
    let primer = s.chars().next()?;  // None si está vacío
    Some(primer.to_ascii_uppercase())
}
```

---

## Convertir Entre Option y Result

```rust
let opt: Option<i32> = Some(42);

// Option a Result
let res: Result<i32, &str> = opt.ok_or("No hay valor");

// Result a Option
let res: Result<i32, &str> = Ok(42);
let opt: Option<i32> = res.ok();  // Descarta el error
```

---

## Patrón Común: Cadena de Operaciones

```rust
fn procesar_usuario(id: &str) -> Result<String, String> {
    let id_num: u32 = id
        .parse()
        .map_err(|_| "ID inválido")?;
    
    let usuario = buscar_usuario(id_num)
        .ok_or("Usuario no encontrado")?;
    
    let email = usuario.email
        .ok_or("Usuario sin email")?;
    
    Ok(format!("Email: {}", email))
}
```

---

## Resumen

| Tipo | Uso | Variantes |
|------|-----|-----------|
| `Option<T>` | Valor opcional | `Some(T)`, `None` |
| `Result<T, E>` | Puede fallar | `Ok(T)`, `Err(E)` |

| Método | Option | Result |
|--------|--------|--------|
| `unwrap()` | ✅ | ✅ |
| `unwrap_or()` | ✅ | ✅ |
| `map()` | ✅ | ✅ |
| `and_then()` | ✅ | ✅ |
| `?` | ✅ | ✅ |
| `ok_or()` | ✅ | - |
| `ok()` | - | ✅ |

---

## 🧪 Ejercicio Mental

Refactoriza usando `?`:

```rust
fn obtener_valor(mapa: &HashMap<&str, Option<i32>>) -> Option<i32> {
    match mapa.get("clave") {
        Some(opt) => match opt {
            Some(n) => Some(*n * 2),
            None => None,
        },
        None => None,
    }
}
```

<details>
<summary>Ver respuesta</summary>

```rust
fn obtener_valor(mapa: &HashMap<&str, Option<i32>>) -> Option<i32> {
    let valor = mapa.get("clave")?.as_ref()?;
    Some(*valor * 2)
}
```

</details>

---

## 📚 Recursos

- [Option en std](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Result en std](https://doc.rust-lang.org/std/result/enum.Result.html)
