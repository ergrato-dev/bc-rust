# Práctica 01 — macro_rules! Básico

## 🎯 Objetivo

Crear tres macros declarativas con `macro_rules!`: una macro de saludo con múltiples ramas, una macro `map!` para construir `HashMap` con pares clave-valor, y una `assert_matches!` con soporte para guards.

## 📋 Instrucciones

Implementa el cuerpo de las tres macros en `src/main.rs`. Cada una tiene marcadores `todo!()` donde debes añadir tu código:

1. **`saludar!`** — Dos ramas: una para un argumento (`nombre`) y otra para dos (`nombre`, `contexto`).

2. **`map!`** — Usa repetición `$(...),* $(,)?` para aceptar 0 o más pares `clave => valor` con coma final opcional. Cuerpo: crear `HashMap`, hacer `insert` para cada par con `$(...)*`, retornar el map.

3. **`assert_matches!`** — Dos ramas:
   - Básica: `($expr, $pat)` — usa `match` con `_ => panic!(...)`
   - Con guard: `($expr, $pat if $guard, $($msg)+)` — incluye el guard en el `match`

## ✅ Criterios de Aceptación

- [ ] El programa compila sin warnings
- [ ] `cargo test -p practice-01-macro-rules-basico` pasa todos los tests
- [ ] `cargo clippy -p practice-01-macro-rules-basico -- -D warnings` sin errores
- [ ] `map!{}` retorna un `HashMap` vacío
- [ ] `map!{ "a" => 1, }` acepta coma final sin error
- [ ] `assert_matches!(None, Some(_))` hace pánico

## 💡 Pistas

<details>
<summary>Pista 1 — map!</summary>

El cuerpo de `map!` necesita un bloque doble `{{ }}` para que sea una expresión:

```rust
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $(m.insert($k, $v);)*
        m
    }};
}
```
</details>

<details>
<summary>Pista 2 — assert_matches! básico</summary>

```rust
($expresion:expr, $patron:pat) => {
    match $expresion {
        $patron => {}
        otro => panic!("assert_matches! falló: {:?} no coincide con el patrón", otro),
    }
};
```
</details>

<details>
<summary>Pista 3 — assert_matches! con guard</summary>

El guard va dentro del arm del match:

```rust
($expresion:expr, $patron:pat if $guard:expr, $($msg:tt)+) => {
    match $expresion {
        $patron if $guard => {}
        _ => panic!($($msg)+),
    }
};
```
</details>

## 🔗 Referencias

- [The Rust Reference — Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Teoría: 02-macro-rules.md](../../1-theory/02-macro-rules.md)
