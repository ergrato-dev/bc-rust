# 📖 Glosario - Semana 04: Ownership y Borrowing

## B

### Borrow (Préstamo)
El acto de crear una referencia a un valor sin tomar ownership. El valor sigue perteneciendo al dueño original.
```rust
let s = String::from("hola");
let r = &s;  // r "toma prestado" s
```

### Borrow Checker
Componente del compilador de Rust que verifica las reglas de ownership y borrowing en tiempo de compilación.

### Borrowing Rules (Reglas de Préstamo)
1. Puedes tener múltiples referencias inmutables (`&T`)
2. O una única referencia mutable (`&mut T`)
3. Pero nunca ambas al mismo tiempo

## C

### Clone
Trait que permite crear una copia profunda de un valor. Requiere llamada explícita con `.clone()`.
```rust
let s1 = String::from("hola");
let s2 = s1.clone();  // Copia explícita
```

### Copy
Trait que indica que un tipo puede copiarse automáticamente (bit a bit). Solo para tipos simples en el stack.
```rust
let x = 5;
let y = x;  // x se copia, ambos válidos
```

## D

### Dangling Reference (Referencia Colgante)
Una referencia a memoria que ya fue liberada. Rust previene esto en tiempo de compilación.
```rust
// NO COMPILA - referencia a variable local
fn dangling() -> &String {
    let s = String::from("hola");
    &s  // ❌ s se destruye, referencia inválida
}
```

### Data Race (Carrera de Datos)
Situación donde múltiples accesos a datos ocurren simultáneamente, al menos uno es escritura, y no hay sincronización. Imposible en safe Rust.

### Drop
Trait que define qué hacer cuando un valor sale del scope. Rust lo llama automáticamente.
```rust
impl Drop for MiTipo {
    fn drop(&mut self) {
        // Liberar recursos
    }
}
```

## H

### Heap
Región de memoria para datos de tamaño dinámico o desconocido en compilación. Más lento que el stack pero más flexible.

## I

### Immutable Reference (Referencia Inmutable)
Referencia que solo permite lectura (`&T`). Pueden existir múltiples simultáneamente.
```rust
let r = &valor;  // Solo lectura
```

## L

### Lifetime (Tiempo de Vida)
La región del código durante la cual una referencia es válida. Relacionado con scopes.

## M

### Move (Movimiento)
Transferencia de ownership de una variable a otra. La variable original se invalida.
```rust
let s1 = String::from("hola");
let s2 = s1;  // s1 se "mueve" a s2
// s1 ya no es válido
```

### Mutable Reference (Referencia Mutable)
Referencia que permite lectura y escritura (`&mut T`). Solo puede existir una a la vez.
```rust
let r = &mut valor;  // Lectura + escritura
```

## N

### NLL (Non-Lexical Lifetimes)
Característica del borrow checker que analiza el último uso de una referencia, no solo el scope léxico.
```rust
let r1 = &s;
println!("{}", r1);  // Último uso de r1
let r2 = &mut s;     // ✅ OK gracias a NLL
```

## O

### Owner (Dueño)
La variable que posee un valor y es responsable de liberarlo cuando sale del scope.

### Ownership (Propiedad)
Sistema de Rust para gestionar memoria. Cada valor tiene exactamente un dueño a la vez.

## R

### RAII (Resource Acquisition Is Initialization)
Patrón donde los recursos se adquieren al crear un objeto y se liberan al destruirlo. Rust lo implementa con Drop.

### Reference (Referencia)
Puntero que permite acceder a un valor sin tomar ownership. Puede ser inmutable (`&T`) o mutable (`&mut T`).

## S

### Scope (Ámbito)
Región del código donde una variable es válida, generalmente delimitada por llaves `{}`.
```rust
{
    let x = 5;  // x válido aquí
}  // x sale del scope
```

### Stack
Región de memoria para datos de tamaño conocido en compilación. Rápido, automático, LIFO.

## U

### Use-After-Free
Bug de memoria donde se accede a datos ya liberados. Imposible en safe Rust gracias al borrow checker.

---

## 🔤 Símbolos

| Símbolo | Significado |
|---------|-------------|
| `&T` | Referencia inmutable a T |
| `&mut T` | Referencia mutable a T |
| `*r` | Desreferenciación (acceder al valor) |
| `'a` | Anotación de lifetime |

---

## 📊 Tabla Resumen

| Operación | Síntaxis | Original válido |
|-----------|----------|-----------------|
| Move | `let y = x;` | ❌ (si no es Copy) |
| Copy | `let y = x;` | ✅ (si es Copy) |
| Clone | `let y = x.clone();` | ✅ |
| Borrow | `let r = &x;` | ✅ |
| Borrow mut | `let r = &mut x;` | ✅ (exclusivo) |
