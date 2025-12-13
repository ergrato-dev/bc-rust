# Práctica 02: Modos de Captura

## 🎯 Objetivos

- Entender los tres modos de captura
- Saber cuándo usar `move`
- Predecir qué modo elige el compilador

## 📋 Modos de Captura

| Modo | Captura | Trait | Uso |
|------|---------|-------|-----|
| Referencia | `&T` | `Fn` | Solo leer |
| Ref. Mutable | `&mut T` | `FnMut` | Modificar |
| Por valor | `T` | `FnOnce` | Ownership |

## 📝 Ejercicios

### Ejercicio 1: Captura por Referencia (3 puntos)

Crea un closure que lea una variable sin modificarla.

```rust
let mensaje = String::from("Hola");
let imprimir = || println!("{}", mensaje);
imprimir();
imprimir();
println!("{}", mensaje); // Debe seguir disponible
```

### Ejercicio 2: Captura por Referencia Mutable (4 puntos)

Crea un closure que modifique un buffer.

```rust
let mut buffer = String::new();
let mut agregar = |s| buffer.push_str(s);
agregar("Hola");
agregar(" Mundo");
```

### Ejercicio 3: Captura con Move (3 puntos)

Usa `move` para transferir ownership.

```rust
let datos = vec![1, 2, 3];
let suma = move || datos.iter().sum();
```

### Ejercicio 4: Múltiples Capturas (3 puntos)

Un closure que capture múltiples variables.

### Ejercicio 5: Move con Tipos Copy (2 puntos)

Demuestra que `move` copia tipos `Copy`.

## 🧪 Tests

```bash
cargo test
```

## 💡 Pistas

1. **Referencia**: el closure solo lee → `Fn`
2. **Ref. Mutable**: closure y variable deben ser `mut`
3. **Move**: usa `move ||` antes del closure
4. **Copy**: `i32`, `f64`, `bool`, `char` se copian con move

## ✅ Verificación

```bash
cargo run
cargo test
cargo clippy
```
