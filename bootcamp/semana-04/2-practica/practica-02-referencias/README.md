# Práctica 02: Referencias

## 🎯 Objetivo

Dominar el uso de referencias inmutables (`&T`) y mutables (`&mut T`).

## 📋 Ejercicios

### Ejercicio 1: Referencia Inmutable

Completa la función para que calcule la longitud sin tomar ownership:

```rust
fn longitud(s: /* ¿qué tipo? */) -> usize {
    // TODO
}

fn main() {
    let texto = String::from("Rust es genial");
    let len = longitud(/* ¿cómo llamar? */);
    
    println!("'{}' tiene {} caracteres", texto, len);
}
```

### Ejercicio 2: Referencia Mutable

Completa la función que modifica un String:

```rust
fn agregar_signo(s: /* ¿qué tipo? */) {
    // TODO: agregar "!" al final
}

fn main() {
    let mut saludo = String::from("Hola");
    agregar_signo(/* ¿cómo llamar? */);
    
    println!("{}", saludo); // Debería imprimir "Hola!"
}
```

### Ejercicio 3: Múltiples Referencias

¿Cuáles de estos bloques compilan?

```rust
// Bloque A
let s = String::from("hola");
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);

// Bloque B
let mut s = String::from("hola");
let r1 = &mut s;
let r2 = &mut s;
println!("{} {}", r1, r2);

// Bloque C
let mut s = String::from("hola");
let r1 = &s;
let r2 = &mut s;
println!("{}", r1);
```

### Ejercicio 4: Función con Múltiples Referencias

Implementa una función que compare dos strings por longitud:

```rust
fn mas_larga(s1: &String, s2: &String) -> &String {
    // TODO: retornar la referencia al string más largo
}
```

**Nota**: Este ejercicio tiene un problema sutil. ¿Puedes identificarlo?

---

## ✅ Criterios de Éxito

- [ ] Usas `&T` cuando solo necesitas leer
- [ ] Usas `&mut T` cuando necesitas modificar
- [ ] Entiendes las reglas de múltiples referencias
- [ ] Puedes identificar errores del borrow checker

---

## 💡 Pistas

<details>
<summary>Pista Ejercicio 4</summary>

El compilador necesita saber cuánto tiempo vive la referencia retornada.
Esto se relaciona con **lifetimes** (semana posterior).
Por ahora, retorna `String` en lugar de `&String`.

</details>
