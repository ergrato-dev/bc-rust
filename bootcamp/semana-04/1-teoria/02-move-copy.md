# 🔄 Move Semantics vs Copy Semantics

> **¿Por qué algunos tipos se mueven y otros se copian?**

---

## El Problema del Move

Ya vimos que `String` se mueve:

```rust
let s1 = String::from("hola");
let s2 = s1;
// s1 ya no es válido ❌
```

Pero... ¿qué pasa con los números?

```rust
let x = 5;
let y = x;
println!("{} y {}", x, y);  // ✅ ¡Ambos son válidos!
```

**¿Por qué?** Porque los enteros implementan el trait `Copy`.

---

## El Trait Copy

Los tipos que implementan `Copy` se **copian automáticamente** en lugar de moverse:

```rust
let x = 5;      // i32 implementa Copy
let y = x;      // Se COPIA, no se mueve

// Ambos son válidos:
println!("x = {}, y = {}", x, y);
```

---

## ¿Qué Tipos son Copy?

### ✅ Tipos Copy (se copian)

| Tipo | Ejemplo | Razón |
|------|---------|-------|
| Enteros | `i32`, `u64`, `isize` | Tamaño fijo, stack |
| Flotantes | `f32`, `f64` | Tamaño fijo, stack |
| Booleanos | `bool` | 1 byte |
| Caracteres | `char` | 4 bytes (Unicode) |
| Tuplas* | `(i32, f64)` | *Si todos sus elementos son Copy |
| Arrays* | `[i32; 5]` | *Si sus elementos son Copy |
| Referencias | `&T` | Solo el puntero, no el dato |

### ❌ Tipos NO Copy (se mueven)

| Tipo | Razón |
|------|-------|
| `String` | Datos en heap |
| `Vec<T>` | Datos en heap |
| `Box<T>` | Puntero a heap |
| `HashMap<K, V>` | Estructura compleja en heap |
| Structs* | *Por defecto no son Copy |

---

## La Regla de Copy

Un tipo puede ser `Copy` **solo si**:

1. Todos sus campos son `Copy`
2. No implementa `Drop` (liberación de recursos)

```rust
// ✅ Puede ser Copy - todos los campos son Copy
struct Punto {
    x: i32,
    y: i32,
}

// ❌ NO puede ser Copy - String no es Copy
struct Usuario {
    nombre: String,  // ← Esto impide Copy
    edad: u32,
}
```

---

## Implementar Copy en Structs

Para hacer un struct `Copy`, deriva ambos traits:

```rust
#[derive(Copy, Clone)]  // Requiere Clone también
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Punto { x: 10, y: 20 };
    let p2 = p1;  // Se COPIA
    
    // Ambos son válidos:
    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);
}
```

---

## Clone: Copia Explícita

Para tipos que NO son `Copy`, usamos `.clone()`:

```rust
let s1 = String::from("hola");
let s2 = s1.clone();  // Copia EXPLÍCITA

// Ambos son válidos:
println!("s1 = {}, s2 = {}", s1, s2);
```

### Copy vs Clone

| Característica | Copy | Clone |
|----------------|------|-------|
| Copia | Implícita (automática) | Explícita (`.clone()`) |
| Costo | Barato (bit a bit) | Puede ser caro |
| Tipos | Solo stack | Cualquier tipo |
| Sintaxis | `let y = x;` | `let y = x.clone();` |

---

## Visualizando Copy vs Move

### Copy (tipos simples)

```
ANTES:                DESPUÉS:
┌─────────┐          ┌─────────┐ ┌─────────┐
│ x = 5   │    →     │ x = 5   │ │ y = 5   │
└─────────┘          └─────────┘ └─────────┘
                     (ambos válidos)
```

### Move (tipos heap)

```
ANTES:                DESPUÉS:
┌─────────┐          ┌─────────┐ ┌─────────┐
│ s1 ─────┼──→ HEAP  │ INVÁLIDO│ │ s2 ─────┼──→ HEAP
└─────────┘          └─────────┘ └─────────┘
                     (solo s2 válido)
```

### Clone (copia explícita)

```
ANTES:                DESPUÉS:
┌─────────┐          ┌─────────┐ ┌─────────┐
│ s1 ─────┼──→ HEAP1 │ s1 ─────┼──→ HEAP1  
└─────────┘          └─────────┘
                     ┌─────────┐
                     │ s2 ─────┼──→ HEAP2 (NUEVO)
                     └─────────┘
                     (ambos válidos, memoria duplicada)
```

---

## ¿Cuándo Usar Clone?

### ✅ Casos válidos para clone()

```rust
// 1. Necesitas dos copias independientes
let original = vec![1, 2, 3];
let copia = original.clone();

// 2. Pasar a función que toma ownership
fn procesar(v: Vec<i32>) { /* ... */ }
procesar(original.clone());
println!("{:?}", original);  // Todavía válido

// 3. Evitar referencias complicadas (temporalmente)
```

### ❌ Evitar clone() innecesario

```rust
// MAL: Clonar cuando no es necesario
fn longitud(s: String) -> usize {
    s.len()
}
let texto = String::from("hola");
let len = longitud(texto.clone());  // ❌ Desperdicio

// MEJOR: Usar referencia
fn longitud(s: &String) -> usize {
    s.len()
}
let len = longitud(&texto);  // ✅ Sin copia
```

---

## Tipos Comunes y su Comportamiento

```rust
// Copy - se copian implícitamente
let a: i32 = 10;
let b = a;  // Copy

let c: bool = true;
let d = c;  // Copy

let e: char = 'R';
let f = e;  // Copy

// Move - se mueven
let s1: String = String::from("hola");
let s2 = s1;  // Move

let v1: Vec<i32> = vec![1, 2, 3];
let v2 = v1;  // Move

// Clone - copia explícita
let s3 = s2.clone();  // Clone
let v3 = v2.clone();  // Clone
```

---

## Resumen

| Operación | Semántica | Validez Original | Costo |
|-----------|-----------|------------------|-------|
| Asignación Copy | Copia bits | ✅ Válido | Bajo |
| Asignación Move | Mueve ownership | ❌ Inválido | Cero |
| `.clone()` | Copia profunda | ✅ Válido | Variable |

---

## 🧪 Ejercicio Mental

¿Cuáles de estas asignaciones compilan?

```rust
let a = 5;
let b = a;
println!("{}", a);  // ¿?

let s = String::from("rust");
let t = s;
println!("{}", s);  // ¿?

let p = (1, 2);
let q = p;
println!("{:?}", p);  // ¿?
```

<details>
<summary>Ver respuesta</summary>

```rust
println!("{}", a);   // ✅ Compila - i32 es Copy
println!("{}", s);   // ❌ ERROR - String se movió
println!("{:?}", p); // ✅ Compila - tupla de Copy es Copy
```

</details>

---

## 📚 Siguiente

[Referencias y Borrowing →](03-referencias-borrowing.md)
