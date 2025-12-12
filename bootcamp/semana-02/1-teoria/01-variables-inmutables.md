# 📖 Variables Inmutables

## 🎯 Objetivo de Aprendizaje

Comprender por qué Rust usa inmutabilidad por defecto y cómo declarar variables inmutables.

---

## 📚 Contenido

### ¿Qué es una Variable?

Una **variable** es un nombre asociado a un valor almacenado en memoria.

```rust
let edad = 25;
//  ↑    ↑
//  │    └── Valor
//  └─────── Nombre de la variable
```

### Inmutabilidad por Defecto

En Rust, todas las variables son **inmutables por defecto**:

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);
    
    // x = 10;  // ❌ ERROR: cannot assign twice to immutable variable
}
```

![Diagrama Inmutabilidad](../0-assets/01-inmutabilidad.svg)

### ¿Por qué Inmutable por Defecto?

| Razón | Explicación |
|-------|-------------|
| **Seguridad** | Evita cambios accidentales |
| **Predecibilidad** | El valor no cambia inesperadamente |
| **Concurrencia** | Facilita código thread-safe |
| **Optimización** | El compilador puede optimizar mejor |

### El Error de Inmutabilidad

Cuando intentas modificar una variable inmutable:

```rust
fn main() {
    let x = 5;
    x = 10;  // ❌ Error
}
```

**Error del compilador:**
```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x = 10;
  |     ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

> 💡 Lee siempre el `help:` - Rust te dice exactamente cómo solucionarlo.

---

## 🔍 Declaración de Variables

### Sintaxis Básica

```rust
let nombre_variable = valor;
```

### Con Tipo Explícito

```rust
let nombre_variable: Tipo = valor;
```

### Ejemplos

```rust
fn main() {
    // Inferencia de tipo
    let numero = 42;           // i32 inferido
    let decimal = 3.14;        // f64 inferido
    let activo = true;         // bool inferido
    let letra = 'R';           // char inferido
    let texto = "Hola";        // &str inferido
    
    // Tipo explícito
    let edad: u32 = 25;
    let precio: f64 = 99.99;
    let encontrado: bool = false;
    
    println!("numero: {}", numero);
    println!("edad: {}", edad);
}
```

---

## 💡 Buenas Prácticas

### 1. Usa Inmutabilidad Siempre que Puedas

```rust
// ✅ Preferido: inmutable
let total = calcular_total();

// ⚠️ Solo si es necesario: mutable
let mut contador = 0;
```

### 2. Nombres Descriptivos

```rust
// ❌ Evita
let x = 25;
let n = "Juan";

// ✅ Preferido
let edad = 25;
let nombre = "Juan";
```

### 3. snake_case para Variables

```rust
// ✅ Correcto en Rust
let nombre_completo = "Juan Pérez";
let numero_de_telefono = "123456789";

// ❌ Incorrecto (pero compila con warning)
let nombreCompleto = "Juan Pérez";  // camelCase
let NombreCompleto = "Juan Pérez";  // PascalCase
```

---

## 🧪 Ejercicio Rápido

¿Cuál de estos códigos compila correctamente?

**Opción A:**
```rust
fn main() {
    let x = 5;
    let y = x + 10;
    println!("{}", y);
}
```

**Opción B:**
```rust
fn main() {
    let x = 5;
    x = x + 10;
    println!("{}", x);
}
```

<details>
<summary>Ver respuesta</summary>

**Opción A es correcta** ✅

- Crea `x` con valor 5
- Crea `y` con valor 15 (usando `x`)
- No modifica `x`, solo lo lee

**Opción B tiene error** ❌

- Intenta modificar `x` que es inmutable
- Necesitaría `let mut x = 5;`

</details>

---

## 📌 Resumen

| Concepto | Descripción |
|----------|-------------|
| `let x = 5;` | Variable inmutable |
| Inmutable | No puede cambiar después de asignarse |
| Por defecto | Rust prefiere seguridad sobre conveniencia |
| Error E0384 | Intentaste modificar variable inmutable |

---

## 🔗 Siguiente

[Variables Mutables →](./02-variables-mutables.md)
