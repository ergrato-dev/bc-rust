# Glosario - Semana 11: Lifetimes

## Términos Fundamentales

### Lifetime (Tiempo de Vida)
El período durante el cual una referencia es válida. En Rust, los lifetimes aseguran que las referencias no sobrevivan a los datos a los que apuntan.

```rust
// 'a es un parámetro de lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

### Borrow (Préstamo)
El acto de tomar una referencia a un valor sin tomar ownership. Puede ser inmutable (`&`) o mutable (`&mut`).

```rust
let s = String::from("hello");
let r = &s;        // préstamo inmutable
let m = &mut s;    // préstamo mutable (requiere mut)
```

### Dangling Reference (Referencia Colgante)
Una referencia que apunta a memoria que ya ha sido liberada. Rust previene esto en tiempo de compilación.

```rust
// ❌ Error: dangling reference
fn dangling() -> &String {
    let s = String::from("hello");
    &s  // 's' se libera aquí
}
```

---

## Parámetros de Lifetime

### Lifetime Parameter (Parámetro de Lifetime)
Un nombre genérico que representa un lifetime, siempre comienza con apóstrofe (`'`).

```rust
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
//     ^^  ^^  parámetros de lifetime
```

### Lifetime Annotation (Anotación de Lifetime)
Sintaxis explícita para especificar el lifetime de una referencia.

```rust
&'a str      // referencia con lifetime 'a
&'a mut str  // referencia mutable con lifetime 'a
```

### Named Lifetime (Lifetime con Nombre)
Un lifetime que tiene un nombre específico asignado por el programador.

```rust
fn compare<'long, 'short>(
    x: &'long str,
    y: &'short str
) -> &'long str
```

---

## Lifetimes Especiales

### 'static
El lifetime más largo posible. Los datos con lifetime `'static` viven durante toda la ejecución del programa.

```rust
// String literal tiene lifetime 'static
let s: &'static str = "hello world";

// Bound 'static en trait
fn print_it(s: impl Display + 'static)
```

### Anonymous Lifetime ('_)
Un lifetime que el compilador infiere automáticamente.

```rust
// Equivalentes:
fn foo(s: &str) -> &str
fn foo<'a>(s: &'a str) -> &'a str
fn foo(s: &'_ str) -> &'_ str
```

---

## Elisión de Lifetimes

### Lifetime Elision (Elisión de Lifetimes)
Reglas que permiten omitir anotaciones de lifetime en casos comunes.

```rust
// Con elisión (común)
fn first_word(s: &str) -> &str

// Sin elisión (explícito)
fn first_word<'a>(s: &'a str) -> &'a str
```

### Input Lifetime
Lifetime de los parámetros de entrada de una función.

### Output Lifetime
Lifetime del valor de retorno de una función.

### Elision Rules (Reglas de Elisión)
1. Cada parámetro de referencia obtiene su propio lifetime
2. Si hay exactamente un input lifetime, se asigna a todos los output lifetimes
3. Si hay `&self` o `&mut self`, su lifetime se asigna al output

---

## Lifetimes en Estructuras

### Struct Lifetime
Cuando un struct contiene referencias, debe declarar el lifetime de esas referencias.

```rust
struct Excerpt<'a> {
    text: &'a str,  // el struct no puede vivir más que 'text'
}
```

### Lifetime Bound
Restricción que indica que un lifetime debe ser al menos tan largo como otro.

```rust
// 'b debe vivir al menos tanto como 'a
struct Wrapper<'a, 'b: 'a> {
    short: &'a str,
    long: &'b str,
}
```

---

## Conceptos Avanzados

### Variance (Varianza)
Cómo los lifetimes se relacionan en términos de subtipado.

- **Covariant**: `'long` puede usarse donde se espera `'short`
- **Contravariant**: `'short` puede usarse donde se espera `'long`
- **Invariant**: El lifetime debe coincidir exactamente

### Subtyping (Subtipado)
`'longer: 'shorter` significa que `'longer` es un subtipo de `'shorter` (vive más tiempo).

```rust
fn example<'long: 'short, 'short>(
    long_ref: &'long str,
    short_ref: &'short str,
)
```

### HRTB (Higher-Ranked Trait Bounds)
Bounds de traits que funcionan para cualquier lifetime.

```rust
// F funciona para CUALQUIER lifetime
fn apply<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str
```

### NLL (Non-Lexical Lifetimes)
Sistema moderno de Rust donde los lifetimes terminan donde se usa la referencia por última vez, no al final del scope.

```rust
let mut x = 5;
let r = &x;
println!("{}", r);  // r termina aquí (NLL)
x = 6;              // OK con NLL, era error antes
```

---

## Borrowing Rules

### Aliasing XOR Mutation
Regla fundamental: puedes tener múltiples referencias inmutables O una referencia mutable, nunca ambas.

```rust
let mut s = String::new();
let r1 = &s;     // OK
let r2 = &s;     // OK
// let m = &mut s; // ERROR: ya hay borrows inmutables
```

### Lifetime Scope
El rango de código donde una referencia es válida.

---

## Errores Comunes

### "does not live long enough"
Error cuando un valor se libera antes de que expire su referencia.

### "cannot borrow as mutable"
Error por violar las reglas de borrowing (aliasing + mutation).

### "missing lifetime specifier"
Error cuando el compilador no puede inferir lifetimes y necesita anotaciones explícitas.

---

## Símbolos y Sintaxis

| Símbolo | Significado |
|---------|-------------|
| `'a` | Parámetro de lifetime |
| `'_` | Lifetime anónimo |
| `'static` | Lifetime estático |
| `&'a T` | Referencia con lifetime 'a |
| `'a: 'b` | 'a vive al menos tanto como 'b |
| `T: 'a` | T es válido durante 'a |
| `for<'a>` | Para cualquier lifetime 'a (HRTB) |
