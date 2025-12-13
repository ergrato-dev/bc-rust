# 🔧 Implementación de Traits

## Sintaxis de Implementación

```rust
impl NombreTrait for TipoConcreto {
    fn metodo(&self) -> ReturnType {
        // implementación
    }
}
```

## Implementación Básica

### Ejemplo: Trait para Cálculos

```rust
trait Calculable {
    fn calcular(&self) -> f64;
}

struct Circulo {
    radio: f64,
}

struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Calculable for Circulo {
    fn calcular(&self) -> f64 {
        std::f64::consts::PI * self.radio * self.radio
    }
}

impl Calculable for Rectangulo {
    fn calcular(&self) -> f64 {
        self.ancho * self.alto
    }
}
```

## Métodos con Diferentes Firmas

### Métodos que Toman `self`

```rust
trait Consumible {
    // Toma ownership - consume el valor
    fn consumir(self) -> String;
}

trait Mutable {
    // Referencia mutable - puede modificar
    fn modificar(&mut self);
}

trait Inmutable {
    // Referencia inmutable - solo lectura
    fn leer(&self) -> String;
}
```

### Ejemplo Completo

```rust
#[derive(Debug)]
struct Contador {
    valor: i32,
}

impl Contador {
    fn new() -> Self {
        Contador { valor: 0 }
    }
}

// Trait para incrementar
trait Incrementable {
    fn incrementar(&mut self);
    fn incrementar_por(&mut self, cantidad: i32);
}

impl Incrementable for Contador {
    fn incrementar(&mut self) {
        self.valor += 1;
    }
    
    fn incrementar_por(&mut self, cantidad: i32) {
        self.valor += cantidad;
    }
}

// Trait para obtener valor
trait Valor {
    fn obtener(&self) -> i32;
}

impl Valor for Contador {
    fn obtener(&self) -> i32 {
        self.valor
    }
}

fn main() {
    let mut contador = Contador::new();
    contador.incrementar();
    contador.incrementar_por(5);
    println!("Valor: {}", contador.obtener()); // Valor: 6
}
```

## Métodos Default

Los métodos default proporcionan una implementación que los tipos pueden usar o sobrescribir.

```rust
trait Saludable {
    fn nombre(&self) -> &str;
    
    // Método default que usa otro método del trait
    fn saludar(&self) -> String {
        format!("¡Hola, {}!", self.nombre())
    }
    
    // Método default con comportamiento fijo
    fn despedir(&self) -> String {
        format!("¡Adiós, {}!", self.nombre())
    }
}

struct Persona {
    nombre: String,
}

impl Saludable for Persona {
    fn nombre(&self) -> &str {
        &self.nombre
    }
    
    // Sobrescribimos saludar
    fn saludar(&self) -> String {
        format!("¡Buenas, soy {}!", self.nombre())
    }
    
    // despedir() usa la implementación default
}

struct Robot {
    id: String,
}

impl Saludable for Robot {
    fn nombre(&self) -> &str {
        &self.id
    }
    // Usa ambas implementaciones default
}
```

## Implementar Múltiples Traits

Un tipo puede implementar tantos traits como necesite:

```rust
trait Nombrable {
    fn nombre(&self) -> &str;
}

trait Edad {
    fn edad(&self) -> u32;
}

trait Presentable {
    fn presentar(&self) -> String;
}

struct Empleado {
    nombre: String,
    edad: u32,
    puesto: String,
}

impl Nombrable for Empleado {
    fn nombre(&self) -> &str {
        &self.nombre
    }
}

impl Edad for Empleado {
    fn edad(&self) -> u32 {
        self.edad
    }
}

impl Presentable for Empleado {
    fn presentar(&self) -> String {
        format!("{}, {} años, {}", self.nombre, self.edad, self.puesto)
    }
}
```

## Métodos Asociados (Sin `self`)

Los traits también pueden tener métodos asociados:

```rust
trait Creador {
    // Método asociado (constructor)
    fn crear() -> Self;
    
    // Método asociado con parámetros
    fn crear_con_valor(valor: i32) -> Self;
}

struct Punto {
    x: i32,
    y: i32,
}

impl Creador for Punto {
    fn crear() -> Self {
        Punto { x: 0, y: 0 }
    }
    
    fn crear_con_valor(valor: i32) -> Self {
        Punto { x: valor, y: valor }
    }
}

fn main() {
    let p1 = Punto::crear();
    let p2 = Punto::crear_con_valor(5);
    
    println!("p1: ({}, {})", p1.x, p1.y); // p1: (0, 0)
    println!("p2: ({}, {})", p2.x, p2.y); // p2: (5, 5)
}
```

## Traits con Constantes

```rust
trait Configuracion {
    const MAX_INTENTOS: u32;
    const NOMBRE: &'static str;
    
    fn intentos_restantes(&self) -> u32;
}

struct ServidorWeb;

impl Configuracion for ServidorWeb {
    const MAX_INTENTOS: u32 = 3;
    const NOMBRE: &'static str = "WebServer";
    
    fn intentos_restantes(&self) -> u32 {
        Self::MAX_INTENTOS
    }
}
```

## Supertraits (Herencia de Traits)

Un trait puede requerir que otro trait esté implementado:

```rust
// Display es supertrait de Imprimible
trait Imprimible: std::fmt::Display {
    fn imprimir(&self) {
        println!("{}", self);
    }
}

struct Mensaje {
    texto: String,
}

// Primero implementamos Display
impl std::fmt::Display for Mensaje {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "📨 {}", self.texto)
    }
}

// Ahora podemos implementar Imprimible
impl Imprimible for Mensaje {}

fn main() {
    let msg = Mensaje { texto: String::from("Hola") };
    msg.imprimir(); // 📨 Hola
}
```

## Errores Comunes

### Error 1: Olvidar Implementar Método Requerido

```rust
trait Animal {
    fn sonido(&self) -> String;
    fn moverse(&self);
}

struct Pez;

impl Animal for Pez {
    fn sonido(&self) -> String {
        String::from("...")
    }
    // ❌ Error: falta implementar moverse()
}
```

### Error 2: Firma Incorrecta

```rust
trait Sumable {
    fn sumar(&self, otro: &Self) -> Self;
}

struct Numero(i32);

impl Sumable for Numero {
    // ❌ Error: firma no coincide
    fn sumar(&self, otro: i32) -> i32 {
        self.0 + otro
    }
}
```

## Resumen

| Concepto | Descripción |
|----------|-------------|
| `impl T for S` | Implementa trait T para tipo S |
| Método default | Implementación opcional sobrescribible |
| Múltiples traits | Un tipo puede implementar varios |
| Supertraits | Trait que requiere otro trait |
| Métodos asociados | Sin `self`, como constructores |

---

## 🔗 Navegación

| ⬅️ Anterior | 🏠 Índice | ➡️ Siguiente |
|:------------|:--------:|-------------:|
| [Introducción](01-introduccion-traits.md) | [Semana 09](../README.md) | [Derivables](03-traits-derivables.md) |
