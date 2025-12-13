# Práctica 03: Trait Bounds

## 🎯 Objetivo

Dominar la restricción de tipos genéricos usando trait bounds:

- Sintaxis inline `T: Trait`
- Múltiples bounds `T: Trait1 + Trait2`
- Cláusula `where` para bounds complejos
- Traits comunes de la biblioteca estándar

## 📚 Conceptos Clave

### Sintaxis Inline

```rust
fn imprimir<T: Display>(valor: T) {
    println!("{}", valor);
}
```

### Múltiples Bounds

```rust
fn comparar_y_mostrar<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("{} es mayor", a);
    }
}
```

### Cláusula Where

```rust
fn procesar<T, U>(t: T, u: U)
where
    T: Clone + Debug,
    U: Default + Display,
{
    // ...
}
```

## 📝 Ejercicios

### Ejercicio 1: Imprimir con Debug

Implementa una función que imprime cualquier valor que implemente `Debug`.

```rust
fn imprimir_debug<T: ???>(valor: &T)
```

**Trait requerido**: `Debug`

**Dificultad**: ⭐

---

### Ejercicio 2: Comparar y Mostrar

Implementa una función que compara dos valores y muestra el mayor.

```rust
fn mostrar_mayor<T: ???>(a: T, b: T)
```

**Traits requeridos**: `PartialOrd + Display`

**Dificultad**: ⭐⭐

---

### Ejercicio 3: Clonar si es Mayor

Implementa una función que clona el primer valor si es mayor que el segundo.

```rust
fn clonar_si_mayor<T: ???>(a: &T, b: &T) -> Option<T>
```

**Traits requeridos**: `PartialOrd + Clone`

**Dificultad**: ⭐⭐

---

### Ejercicio 4: Contar Ocurrencias

Implementa una función que cuenta cuántas veces aparece cada elemento.

```rust
fn contar_ocurrencias<T: ???>(items: &[T]) -> HashMap<T, usize>
```

**Traits requeridos**: `Hash + Eq + Clone`

**Pista**: Usa `entry().or_insert()` para insertar o actualizar.

**Dificultad**: ⭐⭐⭐

---

### Ejercicio 5: Valor o Default

Implementa una función que devuelve el valor de un `Option` o el default del tipo.

```rust
fn valor_o_default<T: ???>(opcion: Option<T>) -> T
```

**Trait requerido**: `Default`

**Pista**: Usa `unwrap_or_default()`.

**Dificultad**: ⭐⭐

---

### Ejercicio Bonus: Múltiples Bounds con Where

Implementa una función que combina dos valores con diferentes formatos.

```rust
fn combinar_formatos<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Debug + Default,
```

**Dificultad**: ⭐⭐⭐

## 🧪 Ejecución

```bash
# Ejecutar el programa
cargo run

# Ejecutar tests
cargo test

# Ver tests con output
cargo test -- --nocapture
```

## ✅ Criterios de Éxito

- [ ] Todos los tests pasan
- [ ] Trait bounds correctos en cada función
- [ ] Código compila sin warnings
- [ ] Se entiende cuándo usar cada trait

## 💡 Traits Comunes

| Trait | Propósito | Ejemplo |
|-------|-----------|---------|
| `Debug` | Formateo `{:?}` | `println!("{:?}", x)` |
| `Display` | Formateo `{}` | `println!("{}", x)` |
| `Clone` | Duplicar valores | `x.clone()` |
| `Copy` | Copia implícita | Asignación automática |
| `PartialEq` | Comparar `==` | `a == b` |
| `PartialOrd` | Comparar `<`, `>` | `a > b` |
| `Hash` | Calcular hash | `HashMap` keys |
| `Default` | Valor por defecto | `T::default()` |

## 🔗 Recursos

- [The Rust Book - Traits as Parameters](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters)
- [Rust by Example - Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)
