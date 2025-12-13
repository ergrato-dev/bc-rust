# 📚 Structs y Enums Genéricos

## Structs Genéricos

### Sintaxis Básica

```rust
struct Contenedor<T> {
    valor: T,
}

fn main() {
    let entero = Contenedor { valor: 42 };
    let texto = Contenedor { valor: "hola" };
    let flotante = Contenedor { valor: 3.14 };
}
```

### Múltiples Parámetros de Tipo

```rust
struct Par<T, U> {
    primero: T,
    segundo: U,
}

fn main() {
    let p1 = Par { primero: 1, segundo: "uno" };
    let p2 = Par { primero: 3.14, segundo: true };
    let p3 = Par { primero: vec![1, 2], segundo: 'x' };
}
```

### Struct con Mismo Tipo para Múltiples Campos

```rust
struct Punto<T> {
    x: T,
    y: T,
}

fn main() {
    let entero = Punto { x: 5, y: 10 };
    let flotante = Punto { x: 1.0, y: 4.0 };
    
    // ❌ Error: x e y deben ser del mismo tipo
    // let mixto = Punto { x: 5, y: 4.0 };
}

// Solución: dos parámetros de tipo
struct PuntoMixto<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mixto = PuntoMixto { x: 5, y: 4.0 };  // ✅ OK
}
```

## Implementando Métodos

### Métodos para Todos los Tipos

```rust
struct Contenedor<T> {
    valor: T,
}

impl<T> Contenedor<T> {
    // Constructor
    fn new(valor: T) -> Self {
        Self { valor }
    }
    
    // Getter por referencia
    fn valor(&self) -> &T {
        &self.valor
    }
    
    // Getter mutable
    fn valor_mut(&mut self) -> &mut T {
        &mut self.valor
    }
    
    // Consumir y retornar el valor
    fn into_inner(self) -> T {
        self.valor
    }
}
```

### Métodos con Trait Bounds

```rust
impl<T> Contenedor<T> {
    fn new(valor: T) -> Self {
        Self { valor }
    }
}

// Solo para tipos que implementan Clone
impl<T: Clone> Contenedor<T> {
    fn clonar_valor(&self) -> T {
        self.valor.clone()
    }
}

// Solo para tipos que implementan Display
impl<T: std::fmt::Display> Contenedor<T> {
    fn imprimir(&self) {
        println!("Valor: {}", self.valor);
    }
}

// Solo para tipos que implementan Default
impl<T: Default> Contenedor<T> {
    fn vacio() -> Self {
        Self { valor: T::default() }
    }
}
```

### Métodos para Tipos Específicos

```rust
struct Punto<T> {
    x: T,
    y: T,
}

// Para todos los tipos
impl<T> Punto<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Solo para f64
impl Punto<f64> {
    fn distancia_al_origen(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Solo para i32
impl Punto<i32> {
    fn es_origen(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}
```

## Enums Genéricos

### Sintaxis Básica

```rust
enum Opcion<T> {
    Alguno(T),
    Ninguno,
}

fn main() {
    let numero: Opcion<i32> = Opcion::Alguno(42);
    let vacio: Opcion<i32> = Opcion::Ninguno;
}
```

### Múltiples Parámetros de Tipo

```rust
enum Resultado<T, E> {
    Exito(T),
    Fallo(E),
}

fn dividir(a: i32, b: i32) -> Resultado<i32, String> {
    if b == 0 {
        Resultado::Fallo(String::from("División por cero"))
    } else {
        Resultado::Exito(a / b)
    }
}
```

### Option y Result de la Biblioteca Estándar

```rust
// Definición de Option<T>
enum Option<T> {
    Some(T),
    None,
}

// Definición de Result<T, E>
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let numero: Option<i32> = Some(42);
    let resultado: Result<i32, &str> = Ok(100);
}
```

## Implementando Métodos en Enums

```rust
enum Opcion<T> {
    Alguno(T),
    Ninguno,
}

impl<T> Opcion<T> {
    fn es_alguno(&self) -> bool {
        matches!(self, Opcion::Alguno(_))
    }
    
    fn es_ninguno(&self) -> bool {
        matches!(self, Opcion::Ninguno)
    }
    
    fn unwrap(self) -> T {
        match self {
            Opcion::Alguno(v) => v,
            Opcion::Ninguno => panic!("Llamado unwrap en Ninguno"),
        }
    }
    
    fn unwrap_or(self, default: T) -> T {
        match self {
            Opcion::Alguno(v) => v,
            Opcion::Ninguno => default,
        }
    }
}

impl<T: Default> Opcion<T> {
    fn unwrap_or_default(self) -> T {
        match self {
            Opcion::Alguno(v) => v,
            Opcion::Ninguno => T::default(),
        }
    }
}
```

## Métodos que Mezclan Tipos

```rust
struct Punto<T, U> {
    x: T,
    y: U,
}

impl<T, U> Punto<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
    
    // Método que toma otro Punto con tipos diferentes
    fn mezclar<V, W>(self, otro: Punto<V, W>) -> Punto<T, W> {
        Punto {
            x: self.x,
            y: otro.y,
        }
    }
}

fn main() {
    let p1 = Punto::new(5, "hola");
    let p2 = Punto::new(true, 3.14);
    
    let p3 = p1.mezclar(p2);
    // p3 tiene tipo Punto<i32, f64>
    // p3.x = 5 (de p1)
    // p3.y = 3.14 (de p2)
}
```

## Patrones Comunes

### Newtype Pattern con Genéricos

```rust
struct Id<T> {
    valor: u64,
    _marker: std::marker::PhantomData<T>,
}

struct Usuario;
struct Producto;

impl<T> Id<T> {
    fn new(valor: u64) -> Self {
        Self { 
            valor, 
            _marker: std::marker::PhantomData 
        }
    }
}

fn main() {
    let id_usuario: Id<Usuario> = Id::new(1);
    let id_producto: Id<Producto> = Id::new(1);
    
    // ❌ No puedes mezclar tipos aunque el valor sea igual
    // fn procesar_usuario(id: Id<Usuario>) { }
    // procesar_usuario(id_producto); // Error de tipos
}
```

### Builder Pattern Genérico

```rust
struct Configuracion<T> {
    valor: T,
    nombre: String,
    activo: bool,
}

struct ConfigBuilder<T> {
    valor: Option<T>,
    nombre: Option<String>,
    activo: bool,
}

impl<T> ConfigBuilder<T> {
    fn new() -> Self {
        Self {
            valor: None,
            nombre: None,
            activo: true,
        }
    }
    
    fn valor(mut self, v: T) -> Self {
        self.valor = Some(v);
        self
    }
    
    fn nombre(mut self, n: &str) -> Self {
        self.nombre = Some(n.to_string());
        self
    }
    
    fn activo(mut self, a: bool) -> Self {
        self.activo = a;
        self
    }
    
    fn build(self) -> Option<Configuracion<T>> {
        Some(Configuracion {
            valor: self.valor?,
            nombre: self.nombre?,
            activo: self.activo,
        })
    }
}

fn main() {
    let config = ConfigBuilder::new()
        .valor(42)
        .nombre("mi_config")
        .activo(true)
        .build()
        .unwrap();
}
```

## Implementando Traits para Tipos Genéricos

```rust
use std::fmt::{Display, Formatter, Result};

struct Par<T, U> {
    primero: T,
    segundo: U,
}

// Display solo si ambos tipos implementan Display
impl<T: Display, U: Display> Display for Par<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.primero, self.segundo)
    }
}

// Clone solo si ambos tipos implementan Clone
impl<T: Clone, U: Clone> Clone for Par<T, U> {
    fn clone(&self) -> Self {
        Self {
            primero: self.primero.clone(),
            segundo: self.segundo.clone(),
        }
    }
}

// Default solo si ambos tipos implementan Default
impl<T: Default, U: Default> Default for Par<T, U> {
    fn default() -> Self {
        Self {
            primero: T::default(),
            segundo: U::default(),
        }
    }
}
```

## Resumen

| Concepto | Sintaxis |
|----------|----------|
| Struct genérico | `struct Foo<T> { campo: T }` |
| Múltiples tipos | `struct Foo<T, U> { a: T, b: U }` |
| Enum genérico | `enum Bar<T> { Variante(T) }` |
| impl para todos | `impl<T> Foo<T> { }` |
| impl con bound | `impl<T: Clone> Foo<T> { }` |
| impl específico | `impl Foo<i32> { }` |
| Mezclar tipos | `fn mezclar<V, W>(otro: Bar<V, W>)` |
