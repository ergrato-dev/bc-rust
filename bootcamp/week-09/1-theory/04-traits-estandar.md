# 📚 Traits de la Biblioteca Estándar

## Traits Importantes para Implementar

La biblioteca estándar de Rust incluye traits que, al implementarlos, integran tu tipo con el ecosistema de Rust.

## Display

`Display` es para formateo amigable al usuario (usando `{}`).

```rust
use std::fmt;

struct Temperatura {
    celsius: f64,
}

impl fmt::Display for Temperatura {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°C", self.celsius)
    }
}

fn main() {
    let temp = Temperatura { celsius: 23.5 };
    println!("{}", temp);      // 23.5°C
    println!("Hoy: {}", temp); // Hoy: 23.5°C
}
```

### Display vs Debug

| Trait | Formato | Propósito | Derivable |
|-------|---------|-----------|-----------|
| `Debug` | `{:?}` | Depuración | ✅ Sí |
| `Display` | `{}` | Usuario final | ❌ No |

```rust
use std::fmt;

#[derive(Debug)]
struct Producto {
    nombre: String,
    precio: f64,
}

impl fmt::Display for Producto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - ${:.2}", self.nombre, self.precio)
    }
}

fn main() {
    let p = Producto { 
        nombre: String::from("Laptop"), 
        precio: 999.99 
    };
    
    println!("{}", p);   // Laptop - $999.99
    println!("{:?}", p); // Producto { nombre: "Laptop", precio: 999.99 }
}
```

## Default

`Default` proporciona valores por defecto para tu tipo.

```rust
struct Configuracion {
    puerto: u16,
    max_conexiones: u32,
    timeout_ms: u64,
    debug: bool,
}

impl Default for Configuracion {
    fn default() -> Self {
        Configuracion {
            puerto: 8080,
            max_conexiones: 100,
            timeout_ms: 30000,
            debug: false,
        }
    }
}

fn main() {
    // Usar default completo
    let config1 = Configuracion::default();
    
    // Usar default con algunos campos personalizados
    let config2 = Configuracion {
        puerto: 3000,
        debug: true,
        ..Default::default()
    };
    
    println!("Puerto: {}", config2.puerto);           // 3000
    println!("Max conexiones: {}", config2.max_conexiones); // 100
}
```

## From y Into

`From` e `Into` permiten conversiones entre tipos.

### From

```rust
struct Milimetros(u32);
struct Metros(f64);

impl From<Metros> for Milimetros {
    fn from(m: Metros) -> Self {
        Milimetros((m.0 * 1000.0) as u32)
    }
}

fn main() {
    let metros = Metros(1.5);
    let mm = Milimetros::from(metros);
    println!("{} mm", mm.0); // 1500 mm
}
```

### Into (Automático)

Cuando implementas `From`, obtienes `Into` gratis:

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let c = Celsius(100.0);
    
    // Usando From
    let f1 = Fahrenheit::from(Celsius(0.0));
    
    // Usando Into (automático)
    let f2: Fahrenheit = c.into();
    
    println!("{}°F", f1.0); // 32°F
    println!("{}°F", f2.0); // 212°F
}
```

### From para Manejo de Errores

```rust
use std::fmt;

#[derive(Debug)]
struct MiError {
    mensaje: String,
}

impl fmt::Display for MiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.mensaje)
    }
}

impl From<std::io::Error> for MiError {
    fn from(err: std::io::Error) -> Self {
        MiError {
            mensaje: err.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for MiError {
    fn from(err: std::num::ParseIntError) -> Self {
        MiError {
            mensaje: format!("Error de parsing: {}", err),
        }
    }
}

fn leer_numero(s: &str) -> Result<i32, MiError> {
    let num = s.parse::<i32>()?; // ParseIntError se convierte a MiError
    Ok(num)
}
```

## TryFrom y TryInto

Para conversiones que pueden fallar:

```rust
use std::convert::TryFrom;

struct Porcentaje(u8);

impl TryFrom<i32> for Porcentaje {
    type Error = String;
    
    fn try_from(valor: i32) -> Result<Self, Self::Error> {
        if valor < 0 || valor > 100 {
            Err(format!("{} no es un porcentaje válido", valor))
        } else {
            Ok(Porcentaje(valor as u8))
        }
    }
}

fn main() {
    let p1 = Porcentaje::try_from(50);  // Ok(Porcentaje(50))
    let p2 = Porcentaje::try_from(150); // Err("150 no es un porcentaje válido")
    
    match p1 {
        Ok(p) => println!("Porcentaje: {}%", p.0),
        Err(e) => println!("Error: {}", e),
    }
}
```

## AsRef y AsMut

Para referencias genéricas y económicas:

```rust
struct Documento {
    contenido: String,
}

impl AsRef<str> for Documento {
    fn as_ref(&self) -> &str {
        &self.contenido
    }
}

impl AsRef<[u8]> for Documento {
    fn as_ref(&self) -> &[u8] {
        self.contenido.as_bytes()
    }
}

// Función que acepta cualquier cosa convertible a &str
fn contar_palabras<T: AsRef<str>>(texto: T) -> usize {
    texto.as_ref().split_whitespace().count()
}

fn main() {
    let doc = Documento {
        contenido: String::from("Hola mundo Rust"),
    };
    
    println!("Palabras: {}", contar_palabras(&doc)); // 3
    println!("Palabras: {}", contar_palabras("uno dos")); // 2
    println!("Palabras: {}", contar_palabras(String::from("a b c"))); // 3
}
```

## Deref y DerefMut

Permiten tratar tu tipo como otro tipo:

```rust
use std::ops::Deref;

struct CajaString {
    valor: String,
}

impl Deref for CajaString {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        &self.valor
    }
}

fn main() {
    let caja = CajaString {
        valor: String::from("Contenido"),
    };
    
    // Gracias a Deref, podemos usar métodos de str
    println!("Longitud: {}", caja.len());
    println!("Mayúsculas: {}", caja.to_uppercase());
    
    // Deref coercion automática
    fn necesita_str(s: &str) {
        println!("{}", s);
    }
    necesita_str(&caja); // Funciona por Deref
}
```

## Drop

`Drop` se ejecuta cuando un valor sale de scope:

```rust
struct Recurso {
    nombre: String,
}

impl Drop for Recurso {
    fn drop(&mut self) {
        println!("Liberando recurso: {}", self.nombre);
    }
}

fn main() {
    let r1 = Recurso { nombre: String::from("Archivo") };
    {
        let r2 = Recurso { nombre: String::from("Conexión") };
        println!("Dentro del bloque");
    } // r2.drop() se llama aquí
    
    println!("Fuera del bloque");
} // r1.drop() se llama aquí

// Output:
// Dentro del bloque
// Liberando recurso: Conexión
// Fuera del bloque
// Liberando recurso: Archivo
```

## Tabla Resumen

| Trait | Método Principal | Propósito |
|-------|------------------|-----------|
| `Display` | `fmt()` | Formateo para usuarios |
| `Default` | `default()` | Valores por defecto |
| `From<T>` | `from(T)` | Conversión desde T |
| `Into<T>` | `into()` | Conversión hacia T |
| `TryFrom<T>` | `try_from(T)` | Conversión falible desde T |
| `AsRef<T>` | `as_ref()` | Referencia económica |
| `Deref` | `deref()` | Dereference automático |
| `Drop` | `drop()` | Cleanup al destruir |

---

## 🔗 Navegación

| ⬅️ Anterior | 🏠 Índice | ➡️ Siguiente |
|:------------|:--------:|-------------:|
| [Derivables](03-traits-derivables.md) | [Semana 09](../README.md) | [Trait Bounds](05-trait-bounds.md) |
