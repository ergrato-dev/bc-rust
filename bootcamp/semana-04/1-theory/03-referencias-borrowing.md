# 🔗 Referencias y Borrowing

> **¿Cómo usar un valor sin tomar ownership?**

![Referencias y Borrowing](../0-assets/03-referencias-borrowing.svg)

---

## El Problema

Vimos que pasar un valor a una función transfiere ownership:

```rust
fn main() {
    let s = String::from("hola");
    calcular_longitud(s);  // s se mueve
    
    // println!("{}", s);  // ❌ ERROR: s ya no es válido
}

fn calcular_longitud(cadena: String) -> usize {
    cadena.len()
}
```

**¿Y si queremos usar `s` después de la función?**

---

## La Solución: Referencias

Una **referencia** es como un "préstamo" - permite acceder al valor sin tomar ownership:

```rust
fn main() {
    let s = String::from("hola");
    let len = calcular_longitud(&s);  // Préstamo con &
    
    println!("'{}' tiene {} caracteres", s, len);  // ✅ s sigue válido
}

fn calcular_longitud(cadena: &String) -> usize {
    cadena.len()
}
```

---

## Anatomía de una Referencia

```
Variable                Referencia               Datos (Heap)
┌──────────────┐       ┌──────────────┐         ┌───────────────┐
│ s            │       │ &s           │         │               │
│ ┌──────────┐ │       │ ┌──────────┐ │         │ ┌───────────┐ │
│ │ ptr  ────┼─┼───────┼─┼── ptr    │ │         │ │ h│o│l│a   │ │
│ │ len: 4   │ │   ↑   │ └──────────┘ │         │ └───────────┘ │
│ │ cap: 4   │ │   │   └──────────────┘         └───────────────┘
│ └──────────┘ │   │                                    ↑
└──────────────┘   │                                    │
                   └────────────────────────────────────┘
```

La referencia `&s` apunta a `s`, no directamente a los datos.

---

## Crear Referencias

### Referencia inmutable: `&T`

```rust
let s = String::from("hola");
let r = &s;  // r es una referencia a s

println!("{}", r);   // ✅ Leer: OK
// r.push_str("!");  // ❌ Modificar: ERROR
```

### Referencia mutable: `&mut T`

```rust
let mut s = String::from("hola");  // s debe ser mut
let r = &mut s;  // Referencia mutable

r.push_str("!");  // ✅ Modificar: OK
println!("{}", r);  // Imprime: "hola!"
```

---

## Borrowing (Préstamo)

**Borrowing** es el acto de crear una referencia:

```rust
fn main() {
    let s = String::from("hola");
    
    // Préstamo inmutable (borrowing)
    let r1 = &s;      // s presta a r1
    let r2 = &s;      // s presta a r2 también
    
    println!("{} y {}", r1, r2);  // ✅ Múltiples préstamos inmutables
}
```

Así como en la vida real:
- Puedes **prestar** algo a alguien
- Mientras está prestado, tú sigues siendo el dueño
- Cuando termina el préstamo, lo recuperas

---

## Reglas del Borrowing

### Regla 1: Múltiples referencias inmutables ✅

```rust
let s = String::from("hola");

let r1 = &s;
let r2 = &s;
let r3 = &s;

println!("{}, {}, {}", r1, r2, r3);  // ✅ Todo OK
```

### Regla 2: Solo UNA referencia mutable

```rust
let mut s = String::from("hola");

let r1 = &mut s;
// let r2 = &mut s;  // ❌ ERROR: segunda ref mutable

r1.push_str("!");
println!("{}", r1);
```

### Regla 3: No mezclar mutable e inmutable

```rust
let mut s = String::from("hola");

let r1 = &s;      // Ref inmutable
let r2 = &s;      // Otra ref inmutable
// let r3 = &mut s;  // ❌ ERROR: ref mutable mientras hay inmutables

println!("{} y {}", r1, r2);
```

---

## ¿Por Qué Estas Reglas?

### Previenen Data Races

Un **data race** ocurre cuando:
1. Dos o más punteros acceden al mismo dato
2. Al menos uno escribe
3. No hay sincronización

```rust
// Si esto se permitiera:
let mut v = vec![1, 2, 3];
let r = &v[0];       // r apunta al primer elemento
v.push(4);           // ⚠️ Podría reubicar el vector en memoria
println!("{}", r);   // 💥 r apuntaría a memoria inválida
```

Rust **previene esto en compilación**.

---

## El Borrow Checker

El **borrow checker** es el componente del compilador que verifica las reglas:

```rust
fn main() {
    let mut s = String::from("hola");
    
    let r1 = &s;         // ─┐ r1 vive aquí
    let r2 = &s;         //  │ r2 vive aquí
    println!("{}{}", r1, r2); // ─┘ último uso de r1, r2
    
    // r1 y r2 ya no se usan, sus "préstamos" terminaron
    
    let r3 = &mut s;     // ✅ OK: no hay refs inmutables activas
    r3.push_str("!");
    println!("{}", r3);
}
```

El borrow checker analiza el **último uso** de cada referencia.

---

## Referencias en Parámetros

### Patrón común: recibir referencia

```rust
// ANTES: toma ownership (malo)
fn imprimir(s: String) {
    println!("{}", s);
}  // s se destruye aquí

// DESPUÉS: toma referencia (bueno)
fn imprimir(s: &String) {
    println!("{}", s);
}  // Solo el préstamo termina, el valor sigue vivo
```

### Patrón: modificar por referencia

```rust
fn agregar_saludo(s: &mut String) {
    s.push_str(", mundo!");
}

fn main() {
    let mut mensaje = String::from("Hola");
    agregar_saludo(&mut mensaje);
    println!("{}", mensaje);  // "Hola, mundo!"
}
```

---

## Sintaxis Resumen

| Sintaxis | Significado | Puede leer | Puede modificar |
|----------|-------------|------------|-----------------|
| `T` | Ownership | ✅ | ✅ (si mut) |
| `&T` | Referencia inmutable | ✅ | ❌ |
| `&mut T` | Referencia mutable | ✅ | ✅ |

---

## Desreferenciación

Para acceder al valor detrás de una referencia, usa `*`:

```rust
let x = 5;
let r = &x;

println!("{}", r);   // 5 (auto-deref)
println!("{}", *r);  // 5 (explícito)

let mut y = 10;
let m = &mut y;
*m += 5;  // Modificar el valor original
println!("{}", y);  // 15
```

Rust hace **auto-dereferencing** en muchos casos.

---

## Resumen

| Concepto | Descripción |
|----------|-------------|
| `&T` | Referencia inmutable (préstamo de solo lectura) |
| `&mut T` | Referencia mutable (préstamo con escritura) |
| Borrowing | El acto de crear una referencia |
| Borrow Checker | Verifica las reglas en compilación |

---

## 🧪 Ejercicio Mental

¿Este código compila?

```rust
fn main() {
    let mut s = String::from("hola");
    let r1 = &s;
    let r2 = &mut s;
    println!("{}", r1);
}
```

<details>
<summary>Ver respuesta</summary>

❌ **NO compila**

Error: `cannot borrow s as mutable because it is also borrowed as immutable`

`r1` (inmutable) todavía está vivo cuando intentamos crear `r2` (mutable).

</details>

---

## 📚 Siguiente

[Reglas del Borrowing →](04-reglas-borrowing.md)
