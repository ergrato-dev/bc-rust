# Práctica 03: Traits Fn, FnMut, FnOnce

## 🎯 Objetivos

- Entender la jerarquía de traits de función
- Usar bounds de Fn, FnMut, FnOnce
- Elegir el trait correcto para cada situación
- Retornar closures con `Box<dyn Fn>`

## 📋 Jerarquía de Traits

```
FnOnce (más permisivo)
   ↑
FnMut (puede modificar)
   ↑
Fn (solo lectura)
```

| Trait | Captura | Puede llamarse |
|-------|---------|----------------|
| `Fn` | `&T` | Múltiples veces |
| `FnMut` | `&mut T` | Múltiples veces |
| `FnOnce` | `T` | Una vez |

## 📝 Ejercicios

### Ejercicio 1: Aceptar Fn (4 puntos)

```rust
fn aplicar_fn<F>(valor: i32, f: F) -> i32
where
    F: Fn(i32) -> i32
{
    // Tu código aquí
}
```

### Ejercicio 2: Aceptar FnMut (4 puntos)

```rust
fn aplicar_fn_mut_veces<F>(f: &mut F, veces: usize) -> Vec<i32>
where
    F: FnMut() -> i32
{
    // Tu código aquí
}
```

### Ejercicio 3: Aceptar FnOnce (4 puntos)

```rust
fn aplicar_fn_once<F>(f: F) -> i32
where
    F: FnOnce() -> i32
{
    // Tu código aquí
}
```

### Ejercicio 4: Filtrar con Fn (4 puntos)

```rust
fn filtrar<F>(lista: &[i32], predicado: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool
{
    // Tu código aquí
}
```

### Ejercicio 5: Retornar Box<dyn Fn> (4 puntos)

```rust
fn crear_multiplicador_boxed(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    // Tu código aquí - usa Box::new(move |x| ...)
}
```

## 🧪 Tests

```bash
cargo test
```

## 💡 Pistas

1. **Fn**: closure solo lee capturas → más restrictivo para el closure, más flexible para usar
2. **FnMut**: necesita `&mut` en el parámetro
3. **FnOnce**: el closure se consume al llamarlo
4. **Box<dyn Fn>**: permite retornar diferentes closures del mismo tipo

## ✅ Verificación

```bash
cargo run
cargo test
cargo clippy
```
