# Práctica 02 — macro_rules! Avanzado

## 🎯 Objetivo

Implementar macros declarativas avanzadas: recursión en `macro_rules!`, generación de structs e impls completos, y verificación de hygiene.

## 📋 Instrucciones

Implementa los tres macros en `src/main.rs`:

### 1. `maximo!` — Macro recursiva

- **Caso base** (`$x:expr`): retorna `$x` directamente.
- **Caso recursivo** (`$x:expr, $($resto:expr),+`): compara `$x` con `maximo!($($resto),+)` usando un bloque `{ let a = $x; let b = maximo!($($resto),+); if a > b { a } else { b } }`.

### 2. `newtype!` — Generación de código

Genera para `newtype!(Nombre, Tipo)`:
- `struct Nombre(Tipo);`
- `impl From<Tipo> for Nombre { fn from(v: Tipo) -> Self { Self(v) } }`
- `impl From<Nombre> for Tipo { fn from(n: Nombre) -> Self { n.0 } }`
- `impl std::fmt::Display for Nombre { fn fmt(...) { write!(f, "{} {}", self.0, stringify!(Nombre)) } }`

### 3. `impl_ops!` — Operadores aritméticos

Genera para `impl_ops!(Nombre, Tipo)`:
- `impl std::ops::Add for Nombre { type Output = Nombre; fn add(self, rhs: Nombre) -> Nombre { Nombre(self.0 + rhs.0) } }`
- Ídem para `Sub`
- `impl std::ops::Mul<Tipo> for Nombre { ... fn mul(self, rhs: Tipo) -> Nombre { Nombre(self.0 * rhs) } }`

## ✅ Criterios de Aceptación

- [ ] `cargo test -p practice-02-macro-rules-avanzado` pasa todos los tests
- [ ] `maximo!(3, 1, 4, 1, 5)` retorna `5`
- [ ] `newtype!(Litros, f64)` genera Display con formato `"2.5 Litros"`
- [ ] `impl_ops!` genera Add, Sub y Mul correctamente
- [ ] El test de hygiene confirma que variables internas no se filtran

## 💡 Pistas

<details>
<summary>Pista 1 — maximo! recursivo</summary>

```rust
macro_rules! maximo {
    ($x:expr) => { $x };
    ($x:expr, $($resto:expr),+) => {{
        let a = $x;
        let b = maximo!($($resto),+);
        if a > b { a } else { b }
    }};
}
```
</details>

<details>
<summary>Pista 2 — newtype! Display</summary>

```rust
impl std::fmt::Display for $nombre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, stringify!($nombre))
    }
}
```
</details>

<details>
<summary>Pista 3 — impl_ops! Add</summary>

```rust
impl std::ops::Add for $nombre {
    type Output = $nombre;
    fn add(self, rhs: $nombre) -> $nombre {
        $nombre(self.0 + rhs.0)
    }
}
```
</details>

## 🔗 Referencias

- [The Little Book of Rust Macros — Recursion](https://veykril.github.io/tlborm/decl-macros/patterns/tt-muncher.html)
- [Rust Reference — Macro Hygiene](https://doc.rust-lang.org/reference/macros-by-example.html#hygiene)
- [Teoría: 02-macro-rules.md](../../1-theory/02-macro-rules.md)
