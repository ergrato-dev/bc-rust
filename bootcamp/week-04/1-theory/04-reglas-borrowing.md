# ⚖️ Reglas del Borrow Checker

> **El guardián de la seguridad de memoria**

![El Borrow Checker](../0-assets/04-borrow-checker.svg)

---

## Las Reglas Fundamentales

En cualquier momento, puedes tener:

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│   UNA referencia mutable (&mut T)                      │
│                                                         │
│                    ─── O ───                            │
│                                                         │
│   MUCHAS referencias inmutables (&T)                   │
│                                                         │
│              ─── PERO NUNCA AMBAS ───                  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Regla 1: Múltiples Lectores ✅

Puedes tener **infinitas** referencias inmutables simultáneas:

```rust
fn main() {
    let s = String::from("hola");
    
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    let r4 = &s;
    
    println!("{}, {}, {}, {}", r1, r2, r3, r4);  // ✅ OK
}
```

**¿Por qué funciona?** Nadie puede modificar, así que no hay conflictos.

---

## Regla 2: Un Solo Escritor ❌

Solo **una** referencia mutable a la vez:

```rust
fn main() {
    let mut s = String::from("hola");
    
    let r1 = &mut s;
    let r2 = &mut s;  // ❌ ERROR
    
    println!("{}, {}", r1, r2);
}
```

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here
```

---

## Regla 3: No Mezclar Lectores y Escritores ❌

```rust
fn main() {
    let mut s = String::from("hola");
    
    let r1 = &s;      // Ref inmutable
    let r2 = &mut s;  // ❌ ERROR: ref mutable mientras hay inmutable
    
    println!("{}", r1);
}
```

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:14
  |
4 |     let r1 = &s;
  |              -- immutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
6 |     println!("{}", r1);
  |                    -- immutable borrow later used here
```

---

## Non-Lexical Lifetimes (NLL)

El borrow checker analiza el **último uso**, no solo el scope:

```rust
fn main() {
    let mut s = String::from("hola");
    
    let r1 = &s;           // ─┐
    let r2 = &s;           //  │ refs inmutables
    println!("{}{}", r1, r2); // ─┘ último uso de r1, r2
    
    // Aquí r1 y r2 ya "murieron" (NLL)
    
    let r3 = &mut s;       // ✅ OK: no hay refs activas
    r3.push_str("!");
    println!("{}", r3);
}
```

Esto **compila** gracias a NLL (Non-Lexical Lifetimes).

---

## Errores Comunes y Soluciones

### Error 1: Usar después de mover

```rust
// ❌ Problema
let s1 = String::from("hola");
let s2 = s1;
println!("{}", s1);  // ERROR: s1 movido
```

```rust
// ✅ Solución 1: Clonar
let s1 = String::from("hola");
let s2 = s1.clone();
println!("{}", s1);  // OK

// ✅ Solución 2: Usar referencia
let s1 = String::from("hola");
let s2 = &s1;
println!("{}", s1);  // OK
```

---

### Error 2: Modificar mientras se presta

```rust
// ❌ Problema
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);  // ERROR: v prestado inmutablemente
println!("{}", first);
```

```rust
// ✅ Solución: Reorganizar código
let mut v = vec![1, 2, 3];
v.push(4);  // Modificar primero
let first = &v[0];  // Prestar después
println!("{}", first);
```

---

### Error 3: Referencia mutable múltiple

```rust
// ❌ Problema
let mut s = String::from("hola");
let r1 = &mut s;
let r2 = &mut s;  // ERROR
```

```rust
// ✅ Solución: Usar en secuencia
let mut s = String::from("hola");

{
    let r1 = &mut s;
    r1.push_str("!");
}  // r1 termina aquí

let r2 = &mut s;  // OK: r1 ya no existe
r2.push_str("!");
```

---

### Error 4: Retornar referencia a local

```rust
// ❌ Problema: Dangling reference
fn crear_ref() -> &String {
    let s = String::from("hola");
    &s  // ERROR: s se destruye al terminar la función
}
```

```rust
// ✅ Solución: Retornar ownership
fn crear_string() -> String {
    let s = String::from("hola");
    s  // Mover ownership al llamador
}
```

---

## Patrones de Solución

### Patrón 1: Scope Interno

```rust
let mut data = vec![1, 2, 3];

{
    let r = &mut data;
    r.push(4);
}  // r termina aquí

println!("{:?}", data);  // ✅ OK
```

### Patrón 2: Clonar Estratégicamente

```rust
let original = String::from("datos importantes");

// Si realmente necesitas dos copias independientes
let copia = original.clone();
procesar(original);  // Se mueve
usar(copia);         // Usamos la copia
```

### Patrón 3: Referencias en Lugar de Ownership

```rust
// MAL: Tomar ownership innecesariamente
fn procesar(s: String) { /* ... */ }

// BIEN: Solo necesitas leer
fn procesar(s: &String) { /* ... */ }

// MEJOR: Aceptar &str para más flexibilidad
fn procesar(s: &str) { /* ... */ }
```

---

## El Borrow Checker es tu Amigo

El borrow checker **previene bugs reales**:

| Bug Prevenido | Lenguaje Afectado | En Rust |
|---------------|-------------------|---------|
| Use-after-free | C/C++ | ❌ Imposible |
| Double-free | C/C++ | ❌ Imposible |
| Data race | Casi todos | ❌ Imposible |
| Dangling pointer | C/C++ | ❌ Imposible |
| Buffer overflow | C/C++ | ❌ Imposible* |

*Con safe Rust

---

## Lectura de Errores del Compilador

El compilador de Rust da excelentes mensajes:

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  --> src/main.rs:5:5
   |
3  |     let r = &v[0];
   |              - immutable borrow occurs here
4  |     
5  |     v.push(4);
   |     ^^^^^^^^^ mutable borrow occurs here
6  |     
7  |     println!("{}", r);
   |                    - immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
```

**Lee todo el mensaje**: ubicación, causa, y dónde termina el conflicto.

---

## Resumen de Reglas

| Situación | ¿Permitido? |
|-----------|-------------|
| Múltiples `&T` | ✅ Sí |
| Una `&mut T` | ✅ Sí |
| Múltiples `&mut T` | ❌ No |
| `&T` + `&mut T` | ❌ No |
| Referencia después de move | ❌ No |
| Referencia a valor local retornada | ❌ No |

---

## 🧪 Ejercicio Mental

¿Cuáles de estos compilan?

```rust
// A
let mut x = 5;
let r1 = &x;
let r2 = &x;
println!("{} {}", r1, r2);

// B
let mut x = 5;
let r1 = &mut x;
let r2 = &mut x;
println!("{}", r2);

// C
let mut x = 5;
let r1 = &mut x;
println!("{}", r1);
let r2 = &mut x;
println!("{}", r2);
```

<details>
<summary>Ver respuesta</summary>

- **A**: ✅ Compila - múltiples refs inmutables OK
- **B**: ❌ No compila - dos refs mutables simultáneas
- **C**: ✅ Compila - r1 ya no se usa cuando se crea r2 (NLL)

</details>

---

## 📚 Siguiente

[Ownership en Funciones →](05-ownership-funciones.md)
