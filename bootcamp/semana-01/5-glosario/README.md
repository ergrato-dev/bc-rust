# 📖 Glosario - Semana 01

## A

### ASCII
**American Standard Code for Information Interchange**. Sistema de codificación de caracteres que representa texto en computadoras.

---

## B

### Binary (Binario)
Archivo ejecutable compilado que puede correr directamente en el sistema operativo. En Rust, se genera en `target/debug/` o `target/release/`.

### Build
Proceso de compilar código fuente en un ejecutable o biblioteca.

---

## C

### Cargo
El gestor de paquetes y build system oficial de Rust. Maneja dependencias, compilación, testing y más.

```bash
cargo build   # Compilar
cargo run     # Ejecutar
cargo test    # Testear
```

### Cargo.toml
Archivo de configuración de un proyecto Rust. Define nombre, versión, dependencias y metadatos.

```toml
[package]
name = "mi-proyecto"
version = "0.1.0"
edition = "2021"
```

### Clippy
Herramienta de linting para Rust que detecta errores comunes y sugiere mejoras.

```bash
cargo clippy
```

### Compilador
Programa que traduce código fuente a código máquina. El compilador de Rust es `rustc`.

### Crate
Unidad de compilación en Rust. Puede ser una biblioteca (`lib`) o un binario (`bin`). Similar a un "package" en otros lenguajes.

---

## D

### Debug Build
Compilación con información de depuración, sin optimizaciones. Más rápida de compilar pero más lenta de ejecutar.

### Dependencia
Crate externo que tu proyecto necesita. Se declaran en `Cargo.toml` bajo `[dependencies]`.

---

## E

### Edition
Versión del lenguaje Rust. Cada edición (2015, 2018, 2021) puede incluir cambios que rompen compatibilidad. Se especifica en `Cargo.toml`.

### Expresión
Código que produce un valor. En Rust, casi todo es una expresión.

```rust
let x = 5 + 3;  // 5 + 3 es una expresión
```

---

## F

### `fn`
Palabra clave para definir una función en Rust.

```rust
fn saludar() {
    println!("Hola!");
}
```

### Función
Bloque de código reutilizable que puede recibir parámetros y devolver valores.

---

## I

### Inmutable
Que no puede cambiar. Por defecto, todas las variables en Rust son inmutables.

```rust
let x = 5;     // Inmutable
let mut y = 5; // Mutable
```

---

## L

### `let`
Palabra clave para declarar variables.

```rust
let nombre = "Rust";
```

---

## M

### Macro
Código que genera código. Se distinguen de las funciones por el `!` al final.

```rust
println!("Hello");  // println! es un macro
```

### `main()`
Función punto de entrada de todo programa Rust ejecutable.

```rust
fn main() {
    // Aquí empieza la ejecución
}
```

### Mutable
Que puede cambiar. Se declara con `mut`.

```rust
let mut contador = 0;
contador = contador + 1;  // OK
```

---

## O

### Ownership
Sistema de gestión de memoria único de Rust. Cada valor tiene un único "dueño" y se libera cuando el dueño sale del scope.

---

## P

### `println!`
Macro para imprimir texto en la consola con salto de línea.

```rust
println!("Hola, {}!", nombre);
```

---

## R

### Release Build
Compilación optimizada para producción. Más lenta de compilar pero más rápida de ejecutar.

```bash
cargo build --release
```

### Rust
Lenguaje de programación de sistemas enfocado en seguridad, velocidad y concurrencia. Creado por Graydon Hoare en Mozilla.

### rustc
El compilador de Rust. Normalmente se usa a través de Cargo.

### rustfmt
Herramienta oficial para formatear código Rust según las convenciones del lenguaje.

```bash
cargo fmt
```

### rust-analyzer
Servidor de lenguaje (LSP) para Rust que proporciona autocompletado, errores en tiempo real y más en editores como VS Code.

---

## S

### Scope
Región del código donde una variable es válida. En Rust, definido por `{}`.

```rust
{
    let x = 5;  // x existe aquí
}
// x ya no existe aquí
```

### Statement
Instrucción que realiza una acción pero no produce un valor (termina en `;`).

```rust
let x = 5;  // Statement
```

### String Literal
Texto literal definido entre comillas dobles. Es de tipo `&str`.

```rust
let saludo = "Hola, mundo!";
```

---

## T

### Target
Directorio donde Cargo almacena los archivos compilados.

```
target/
├── debug/      # Builds de desarrollo
└── release/    # Builds de producción
```

### TOML
**Tom's Obvious, Minimal Language**. Formato de archivo de configuración usado por Cargo.

---

## V

### Variable
Nombre asociado a un valor en memoria.

```rust
let edad = 25;
```

---

## Z

### Zero-cost Abstraction
Principio de Rust donde las abstracciones de alto nivel no tienen costo en tiempo de ejecución comparado con código de bajo nivel equivalente.

---

## Símbolos

### `//`
Comentario de una línea.

### `///`
Comentario de documentación (rustdoc).

### `{}`
- Delimitadores de bloque de código
- Placeholder en `println!` para interpolación

### `;`
Terminador de statements.

### `::`
Separador de paths (módulos, funciones asociadas).

```rust
std::io::stdin()
```

### `!`
- Al final de un nombre: indica que es un macro
- Operador de negación booleana

---

**Continúa en Semana 02...**
